use minifi_native::{
    ConcurrentOnTrigger, LogLevel, Logger, MinifiError, ProcessContext, ProcessSession, Processor,
};
use std::io::Write;
use std::path::{Path, PathBuf};
use strum_macros::{Display, EnumString, IntoStaticStr, VariantNames};
use walkdir::WalkDir;

mod properties;
mod relationships;
#[cfg(unix)]
mod unix_only_properties;

#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString, VariantNames, IntoStaticStr)]
#[strum(serialize_all = "camelCase")]
enum ConflictResolutionStrategy {
    Fail,
    Replace,
    Ignore,
}

#[derive(Debug)]
struct PutFile<L: Logger> {
    logger: L,
    conflict_resolution_strategy: ConflictResolutionStrategy,
    try_make_dirs: bool,
    maximum_file_count: Option<u64>,
}

impl<L: Logger> PutFile<L> {
    pub(crate) fn directory_is_full(&self, p0: &Path) -> bool {
        if let Some(max_file_count) = self.maximum_file_count
            && let Some(parent) = p0.parent()
        {
            parent.exists() && WalkDir::new(parent).into_iter().count() >= max_file_count as usize
        } else {
            false
        }
    }
}

impl<L: Logger> PutFile<L> {
    fn get_destination_path<C, S>(
        context: &C,
        session: &S,
        ff: &mut S::FlowFile,
    ) -> Result<PathBuf, MinifiError>
    where
        C: ProcessContext,
        S: ProcessSession<FlowFile = C::FlowFile>,
    {
        let directory = context
            .get_property(&properties::DIRECTORY, Some(ff))?
            .expect("required property");

        let file_name = session
            .get_attribute(ff, "filename")
            .unwrap_or("foo.txt".to_string()); // fallback to UUID
        Ok(PathBuf::from(directory + "/" + file_name.as_str()))
    }

    #[cfg(unix)]
    fn prepare_destination(&self, destination: &Path) {
        if let Some(parent) = destination.parent() {
            if self.try_make_dirs {
                std::fs::create_dir_all(parent); // TODO(error handling)
            }
        }
        // TODO(do permissions)
    }

    #[cfg(windows)]
    fn prepare_destination(&self, destination: &Path) {
        todo!("windows implementation");
    }

    fn put_file<C, S>(
        &self,
        session: &mut S,
        destination: &Path,
        ff: &S::FlowFile,
    ) -> Result<(), MinifiError>
    where
        C: ProcessContext,
        S: ProcessSession<FlowFile = C::FlowFile>,
    {
        self.prepare_destination(destination);
        let mut file = std::fs::File::create(destination).map_err(|_| MinifiError::UnknownError)?;

        session.read_in_batches(ff, 1024, |batch| {
            file.write_all(batch).map_err(|_| MinifiError::UnknownError) // TODO(proper mapping)
        })?;

        Ok(())
    }
}

impl<L: Logger> Processor<L> for PutFile<L> {
    type Threading = minifi_native::Concurrent;
    fn new(logger: L) -> Self {
        Self {
            logger,
            conflict_resolution_strategy: ConflictResolutionStrategy::Fail,
            try_make_dirs: true,
            maximum_file_count: None,
        }
    }

    fn log(&self, log_level: LogLevel, message: &str) {
        self.logger.log(log_level, message);
    }

    fn on_schedule<P: ProcessContext>(&mut self, context: &P) -> Result<(), MinifiError> {
        self.conflict_resolution_strategy = context
            .get_property(&properties::CONFLICT_RESOLUTION, None)?
            .expect("required property")
            .parse::<ConflictResolutionStrategy>()?;

        self.try_make_dirs = context
            .get_bool_property(&properties::CREATE_DIRS, None)?
            .expect("required property");

        self.maximum_file_count = context.get_u64_property(&properties::MAX_FILE_COUNT, None)?;

        Ok(())
    }
}

impl<L: Logger> ConcurrentOnTrigger<L> for PutFile<L> {
    fn on_trigger<C, S>(&self, context: &mut C, session: &mut S) -> Result<(), MinifiError>
    where
        C: ProcessContext,
        S: ProcessSession<FlowFile = C::FlowFile>,
    {
        let Some(mut ff) = session.get() else {
            return Ok(());
        };

        let Ok(destination_path) = Self::get_destination_path::<C, S>(context, session, &mut ff)
        else {
            session.transfer(ff, relationships::FAILURE.name);
            return Ok(());
        };

        if self.directory_is_full(&destination_path) {
            // TODO(log warn)
            session.transfer(ff, relationships::FAILURE.name);
            return Ok(());
        }

        if destination_path.exists() {
            match self.conflict_resolution_strategy {
                ConflictResolutionStrategy::Fail => {
                    session.transfer(ff, relationships::FAILURE.name);
                    return Ok(());
                }
                ConflictResolutionStrategy::Replace => {
                    // continue with PutFile operation
                }
                ConflictResolutionStrategy::Ignore => {
                    session.transfer(ff, relationships::SUCCESS.name);
                    return Ok(());
                }
            }
        }

        match self.put_file::<C, S>(session, &destination_path, &ff) {
            Ok(_) => {
                session.transfer(ff, relationships::SUCCESS.name);
                Ok(())
            }
            Err(_e) => {
                session.transfer(ff, relationships::FAILURE.name);
                Ok(())
            }
        }
    }
}

#[cfg(not(test))]
mod register_ctor;

#[cfg(test)]
mod tests;

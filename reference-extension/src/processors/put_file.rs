use minifi_native::{
    ConcurrentOnTrigger, LogLevel, Logger, MinifiError, ProcessContext, ProcessSession, Processor,
};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use strum_macros::{Display, EnumString, IntoStaticStr, VariantNames};
use walkdir::WalkDir;
use crate::processors::put_file::unix_only_properties::{DIRECTORY_PERMISSIONS, PERMISSIONS};

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

#[cfg(unix)]
#[derive(Debug)]
struct PutFileUnixPermissions {
    file_permissions: Option<std::fs::Permissions>,
    directory_permissions: Option<std::fs::Permissions>,
}

#[cfg(unix)]
impl PutFileUnixPermissions {
    fn new() -> Self {
        Self {
            file_permissions: None,
            directory_permissions: None,
        }
    }

    fn set_directory_permissions(&self, path: &Path) -> std::io::Result<()>{
        if let Some(permissions) = self.directory_permissions.as_ref().map(|p| p.clone()) {
            return std::fs::set_permissions(path, permissions);
        }
        Ok(())
    }

    fn set_file_permissions(&self, file: &Path) -> std::io::Result<()> {
        if let Some(permissions) = self.file_permissions.as_ref().map(|p| p.clone()) {
            return std::fs::set_permissions(file, permissions);
        }
        Ok(())
    }
}

#[cfg(windows)]
#[derive(Debug)]
struct PutFileWindowsPermissions {
}

#[cfg(windows)]
impl PutFileWindowsPermissions {
    fn new() -> Self {
        Self {
        }
    }

}


#[derive(Debug)]
struct PutFile<L: Logger> {
    logger: L,
    conflict_resolution_strategy: ConflictResolutionStrategy,
    try_make_dirs: bool,
    maximum_file_count: Option<u64>,
    unix_permissions: PutFileUnixPermissions
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

    fn prepare_destination(&self, destination: &Path) -> std::io::Result<()> {
        if let Some(parent) = destination.parent() {
            if self.try_make_dirs {
                std::fs::create_dir_all(parent)?;
                self.unix_permissions.set_directory_permissions(parent)?;
            }
        }
        Ok(())
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
        match self.prepare_destination(destination) {
            Ok(_) => {}
            Err(err) => {
                self.logger.warn(format!("Failed to prepare destination due to {:?}", err).as_str());
            }
        }
        let mut file = std::fs::File::create(destination).map_err(|e| MinifiError::TriggerError(e.to_string()))?;
        match self.unix_permissions.set_file_permissions(destination) {
            Ok(_) => {}
            Err(err) => {
                self.logger.warn(format!("Failed to set file permissions due to {:?}", err).as_str());
            }
        }

        session.read_in_batches(ff, 1024, |batch| {
            file.write_all(batch).map_err(|e| MinifiError::TriggerError(e.to_string()))
        })?;

        Ok(())
    }

    #[cfg(unix)]
    fn parse_unix_permissions<P: ProcessContext>(&mut self, context: &P) -> Result<(), MinifiError> {
        if let Some(dir_perm_str) = context.get_property(&DIRECTORY_PERMISSIONS, None)? {
            let dir_perm = u32::from_str_radix(&dir_perm_str, 8)?;
            self.unix_permissions.directory_permissions = Some(std::fs::Permissions::from_mode(dir_perm));
        }
        if let Some(perm_str) = context.get_property(&PERMISSIONS, None)? {
            let perm = u32::from_str_radix(&perm_str, 8)?;
            self.unix_permissions.file_permissions = Some(std::fs::Permissions::from_mode(perm));
        }
        Ok(())
    }

    #[cfg(windows)]
    fn parse_unix_permissions<P: ProcessContext>(&mut self, context: &P) -> Result<(), MinifiError> {
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
            unix_permissions: PutFileUnixPermissions::new()
        }
    }

    fn log(&self, log_level: LogLevel, message: &str) {
        self.logger.log(log_level, message);
    }

    fn on_schedule<P: ProcessContext>(&mut self, context: &P) -> Result<(), MinifiError> {
        self.logger.trace(format!("on_schedule: {:?}", self).as_str());
        self.conflict_resolution_strategy = context
            .get_property(&properties::CONFLICT_RESOLUTION, None)?
            .expect("required property")
            .parse::<ConflictResolutionStrategy>()?;

        self.try_make_dirs = context
            .get_bool_property(&properties::CREATE_DIRS, None)?
            .expect("required property");

        self.maximum_file_count = context.get_u64_property(&properties::MAX_FILE_COUNT, None)?;

        self.parse_unix_permissions(context)?;

        Ok(())
    }
}

impl<L: Logger> ConcurrentOnTrigger<L> for PutFile<L> {
    fn on_trigger<C, S>(&self, context: &mut C, session: &mut S) -> Result<(), MinifiError>
    where
        C: ProcessContext,
        S: ProcessSession<FlowFile = C::FlowFile>,
    {
        self.logger.trace(format!("on_trigger: {:?}", self).as_str());
        let Some(mut ff) = session.get() else {
            return Ok(());
        };

        let Ok(destination_path) = Self::get_destination_path::<C, S>(context, session, &mut ff)
        else {
            self.logger.warn("Invalid destination path");
            session.transfer(ff, relationships::FAILURE.name);
            return Ok(());
        };

        if self.directory_is_full(&destination_path) {
            self.logger.warn("Directory is full");
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
            Err(e) => {
                self.logger.warn(format!("Failed to put file due to {:?}", e).as_str());
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

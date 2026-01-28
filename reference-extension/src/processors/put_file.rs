use minifi_native::{ConcurrentOnTrigger, DefaultLogger, LogLevel, Logger, MinifiError, OnTriggerResult, ProcessContext, ProcessSession, Processor};
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

#[cfg(unix)]
#[derive(Debug)]
struct PutFileUnixPermissions {
    file_permissions: Option<std::fs::Permissions>,
    directory_permissions: Option<std::fs::Permissions>,
}

#[cfg(unix)]
impl PutFileUnixPermissions {
    fn set_directory_permissions(&self, path: &Path) -> std::io::Result<()> {
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
struct PutFileUnixPermissions {}

#[cfg(windows)]
impl PutFileUnixPermissions {
    fn set_directory_permissions(&self, _path: &Path) -> std::io::Result<()> {
        Ok(())
    }

    fn set_file_permissions(&self, _file: &Path) -> std::io::Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
struct ScheduledMembers {
    conflict_resolution_strategy: ConflictResolutionStrategy,
    try_make_dirs: bool,
    maximum_file_count: Option<u64>,
    unix_permissions: PutFileUnixPermissions,
}

#[derive(Debug)]
pub(crate) struct PutFile {
    logger: DefaultLogger,
    scheduled_members: Option<ScheduledMembers>,
}

impl PutFile {
    pub(crate) fn directory_is_full(&self, p0: &Path) -> bool {
        let put_file = self
            .scheduled_members
            .as_ref()
            .expect("on_schedule should create GetFileImpl");

        if let Some(max_file_count) = put_file.maximum_file_count
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
        let put_file = self
            .scheduled_members
            .as_ref()
            .expect("on_schedule should create GetFileImpl");

        if let Some(parent) = destination.parent() {
            if put_file.try_make_dirs {
                std::fs::create_dir_all(parent)?;
                put_file
                    .unix_permissions
                    .set_directory_permissions(parent)?;
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
        let put_file = self
            .scheduled_members
            .as_ref()
            .expect("on_schedule should create GetFileImpl");

        match self.prepare_destination(destination) {
            Ok(_) => {}
            Err(err) => {
                self.logger
                    .warn(format!("Failed to prepare destination due to {:?}", err).as_str());
            }
        }
        let mut file = std::fs::File::create(destination)
            .map_err(|e| MinifiError::TriggerError(e.to_string()))?;
        match put_file.unix_permissions.set_file_permissions(destination) {
            Ok(_) => {}
            Err(err) => {
                self.logger
                    .warn(format!("Failed to set file permissions due to {:?}", err).as_str());
            }
        }

        session.read_in_batches(ff, 1024, |batch| {
            file.write_all(batch)
                .map_err(|e| MinifiError::TriggerError(e.to_string()))
        })?;

        Ok(())
    }

    #[cfg(unix)]
    fn parse_unix_permissions<P: ProcessContext>(
        context: &P,
    ) -> Result<PutFileUnixPermissions, MinifiError> {
        use std::os::unix::fs::PermissionsExt;
        let parse_permission = |property: &minifi_native::Property| -> Result<Option<std::fs::Permissions>, MinifiError> {
            Ok(context.get_property(&property, None)?
                .map(|perm_str| { u32::from_str_radix(&perm_str, 8) })
                .transpose()?
                .map(|perm| std::fs::Permissions::from_mode(perm)))
        };
        let file_permissions = parse_permission(&unix_only_properties::PERMISSIONS)?;
        let directory_permissions = parse_permission(&unix_only_properties::DIRECTORY_PERMISSIONS)?;

        Ok(PutFileUnixPermissions {
            file_permissions,
            directory_permissions,
        })
    }

    #[cfg(windows)]
    fn parse_unix_permissions<P: ProcessContext>(
        _context: &P,
    ) -> Result<PutFileUnixPermissions, MinifiError> {
        Ok(PutFileUnixPermissions {})
    }
}

impl Processor for PutFile {
    type Threading = minifi_native::Concurrent;
    fn new(logger: DefaultLogger) -> Self {
        Self {
            logger,
            scheduled_members: None,
        }
    }

    fn log(&self, log_level: LogLevel, message: &str) {
        self.logger.log(log_level, message);
    }

    fn on_schedule<P: ProcessContext>(&mut self, context: &P) -> Result<(), MinifiError> {
        self.logger
            .trace(format!("on_schedule: {:?}", self).as_str());
        let conflict_resolution_strategy = context
            .get_property(&properties::CONFLICT_RESOLUTION, None)?
            .expect("required property")
            .parse::<ConflictResolutionStrategy>()?;

        let try_make_dirs = context
            .get_bool_property(&properties::CREATE_DIRS, None)?
            .expect("required property");

        let maximum_file_count = context.get_u64_property(&properties::MAX_FILE_COUNT, None)?;

        let unix_permissions = PutFile::parse_unix_permissions(context)?;

        self.scheduled_members = Some(ScheduledMembers {
            conflict_resolution_strategy,
            try_make_dirs,
            maximum_file_count,
            unix_permissions,
        });
        Ok(())
    }
}

impl ConcurrentOnTrigger for PutFile {
    fn on_trigger<C, S>(
        &self,
        context: &mut C,
        session: &mut S,
    ) -> Result<OnTriggerResult, MinifiError>
    where
        C: ProcessContext,
        S: ProcessSession<FlowFile = C::FlowFile>,
    {
        let put_file = self
            .scheduled_members
            .as_ref()
            .expect("on_schedule should create GetFileImpl");

        self.logger
            .trace(format!("on_trigger: {:?}", self).as_str());
        let Some(mut ff) = session.get() else {
            return Ok(OnTriggerResult::Yield);
        };

        let Ok(destination_path) = Self::get_destination_path::<C, S>(context, session, &mut ff)
        else {
            self.logger.warn("Invalid destination path");
            session.transfer(ff, relationships::FAILURE.name);
            return Ok(OnTriggerResult::Yield);
        };

        if self.directory_is_full(&destination_path) {
            self.logger.warn("Directory is full");
            session.transfer(ff, relationships::FAILURE.name);
            return Ok(OnTriggerResult::Yield);
        }

        if destination_path.exists() {
            match put_file.conflict_resolution_strategy {
                ConflictResolutionStrategy::Fail => {
                    session.transfer(ff, relationships::FAILURE.name);
                    return Ok(OnTriggerResult::Ok);
                }
                ConflictResolutionStrategy::Replace => {
                    // continue with PutFile operation
                }
                ConflictResolutionStrategy::Ignore => {
                    session.transfer(ff, relationships::SUCCESS.name);
                    return Ok(OnTriggerResult::Ok);
                }
            }
        }

        match self.put_file::<C, S>(session, &destination_path, &ff) {
            Ok(_) => {
                session.transfer(ff, relationships::SUCCESS.name);
                Ok(OnTriggerResult::Ok)
            }
            Err(e) => {
                self.logger
                    .warn(format!("Failed to put file due to {:?}", e).as_str());
                session.transfer(ff, relationships::FAILURE.name);
                Ok(OnTriggerResult::Ok)
            }
        }
    }
}

#[cfg(not(test))]
pub(crate) mod processor_definition;

#[cfg(test)]
mod tests;

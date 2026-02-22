use crate::processors::get_file::output_attributes::{
    ABSOLUTE_PATH_OUTPUT_ATTRIBUTE, FILENAME_OUTPUT_ATTRIBUTE,
};
use crate::processors::get_file::properties::{
    BATCH_SIZE, DIRECTORY, IGNORE_HIDDEN_FILES, KEEP_SOURCE_FILE, MAX_AGE, MAX_SIZE, MIN_AGE,
    MIN_SIZE, RECURSE,
};
use minifi_native::macros::ComponentIdentifier;
use minifi_native::{
    CalculateMetrics, ConstTrigger, Logger, MinifiError, OnTriggerResult, ProcessContext,
    ProcessSession, Schedule,
};
use std::collections::VecDeque;
use std::error;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{Duration, Instant, SystemTime};
use walkdir::{DirEntry, WalkDir};

mod properties;
mod relationships;

#[derive(Debug)]
struct GetFileMetrics {
    accepted_files: u32,
    input_bytes: u64,
}

#[derive(Debug)]
struct DirectoryListing {
    paths: VecDeque<PathBuf>,
    last_polling_time: Option<Instant>,
}

impl DirectoryListing {
    fn new() -> Self {
        Self {
            paths: VecDeque::new(),
            last_polling_time: None,
        }
    }
}

#[derive(Debug, ComponentIdentifier)]
pub(crate) struct GetFileRs {
    recursive: bool,
    keep_source_file: bool,
    input_directory: PathBuf,
    poll_interval: Option<Duration>,
    directory_listing: Mutex<DirectoryListing>,
    batch_size: u64,
    min_size: Option<u64>,
    max_size: Option<u64>,
    min_age: Option<Duration>,
    max_age: Option<Duration>,
    ignore_hidden_files: bool,
    metrics: Mutex<GetFileMetrics>,
}

impl GetFileRs {
    fn is_listing_empty(&self) -> bool {
        let directory_listing = self.directory_listing.lock().unwrap();
        directory_listing.paths.is_empty()
    }

    fn poll_listing(&self, batch_size: u64) -> VecDeque<PathBuf> {
        let mut directory_listings = self.directory_listing.lock().unwrap();

        let mut res = VecDeque::new();
        for _ in 0..batch_size {
            if let Some(path) = directory_listings.paths.pop_back() {
                res.push_back(path);
            } else {
                break;
            }
        }

        res
    }

    fn should_poll(&self) -> bool {
        if self.poll_interval.is_none() {
            return true;
        }
        let directory_listings = self.directory_listing.lock().unwrap();

        if directory_listings.last_polling_time.is_none() {
            return true;
        }
        Instant::now() - directory_listings.last_polling_time.unwrap() > self.poll_interval.unwrap()
    }

    fn perform_listing(&self) {
        let mut directory_listings = self.directory_listing.lock().unwrap();
        let mut walker = WalkDir::new(&self.input_directory);

        if !self.recursive {
            walker = walker.max_depth(1);
        }

        let mut files_added = 0u32;
        let mut bytes_added = 0u64;
        for entry in walker.into_iter().filter_map(Result::ok) {
            if self.entry_matches_criteria(&entry).unwrap_or(false) {
                let file_size = entry.metadata().map(|m| m.len()).unwrap_or(0);
                directory_listings.paths.push_back(entry.into_path());
                files_added += 1;
                bytes_added += file_size;
            }
        }
        let mut metrics = self.metrics.lock().unwrap();
        metrics.accepted_files += files_added;
        metrics.input_bytes += bytes_added;
        directory_listings.last_polling_time = Some(Instant::now());
    }

    fn entry_matches_criteria(&self, dir_entry: &DirEntry) -> Result<bool, Box<dyn error::Error>> {
        let metadata = dir_entry.metadata()?;
        if !metadata.is_file() {
            return Ok(false);
        }
        let age = SystemTime::now().duration_since(metadata.modified()?)?;
        let size = metadata.len();

        if self.min_age.is_some() && age < self.min_age.unwrap() {
            return Ok(false);
        }
        if self.max_age.is_some() && age > self.max_age.unwrap() {
            return Ok(false);
        }
        if self.min_size.is_some() && size < self.min_size.unwrap() {
            return Ok(false);
        }
        if self.max_size.is_some() && size > self.max_size.unwrap() {
            return Ok(false);
        }

        #[cfg(unix)]
        fn is_hidden(path: PathBuf) -> bool {
            path.file_name()
                .and_then(|f| f.to_str())
                .map_or(false, |f| f.starts_with('.'))
        }

        #[cfg(windows)]
        fn is_hidden(path: PathBuf) -> bool {
            use std::fs;
            use std::os::windows::fs::MetadataExt;

            // FILE_ATTRIBUTE_HIDDEN is defined as 0x2 or 2 in Windows API
            const FILE_ATTRIBUTE_HIDDEN: u32 = 2;

            match fs::metadata(&path) {
                Ok(metadata) => (metadata.file_attributes() & FILE_ATTRIBUTE_HIDDEN) != 0,
                Err(_) => false,
            }
        }

        if self.ignore_hidden_files && is_hidden(dir_entry.path().to_path_buf()) {
            return Ok(false);
        }

        Ok(true)
    }

    fn get_single_file<PS: ProcessSession, L: Logger>(
        &self,
        session: &mut PS,
        logger: &L,
        path: PathBuf,
    ) -> Result<(), MinifiError> {
        logger.info(format!("GetFile process {:?}", &path).as_str());
        let mut ff = session
            .create()
            .expect("Successful FlowFile creation is expected");

        if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
            session.set_attribute(&mut ff, FILENAME_OUTPUT_ATTRIBUTE.name, file_name)?;
        } else {
            logger.warn(format!("Couldnt get filename of {:?}", path).as_str());
        }
        session.set_attribute(
            &mut ff,
            ABSOLUTE_PATH_OUTPUT_ATTRIBUTE.name,
            path.to_string_lossy().trim(),
        )?;

        let contents = std::fs::read_to_string(&path).expect("Failed to read file");
        session.write(&mut ff, contents.as_bytes())?;
        if !self.keep_source_file {
            match std::fs::remove_file(&path) {
                Ok(_) => {}
                Err(err) => {
                    logger.warn(format!("Failed to remove source file {:?}", err).as_str());
                }
            }
        }
        session.transfer(ff, relationships::SUCCESS.name)?;
        Ok(())
    }
}

impl Schedule for GetFileRs {
    fn schedule<P: ProcessContext, L: Logger>(context: &P, _logger: &L) -> Result<Self, MinifiError>
    where
        Self: Sized,
    {
        let input_directory: PathBuf = context
            .get_property(&DIRECTORY, None)?
            .expect("Required property")
            .into();
        if !input_directory.is_dir() {
            return Err(MinifiError::ScheduleError(format!(
                "{:?} is not a valid directory",
                input_directory
            )));
        }

        let recursive = context
            .get_bool_property(&RECURSE, None)?
            .expect("Required property");

        let keep_source_file = context
            .get_bool_property(&KEEP_SOURCE_FILE, None)?
            .expect("Required property");

        let poll_interval = context.get_duration_property(&properties::POLLING_INTERVAL, None)?;
        let min_size = context.get_size_property(&MIN_SIZE, None)?;
        let max_size = context.get_size_property(&MAX_SIZE, None)?;
        let min_age = context.get_duration_property(&MIN_AGE, None)?;
        let max_age = context.get_duration_property(&MAX_AGE, None)?;
        let batch_size = context
            .get_u64_property(&BATCH_SIZE, None)?
            .expect("required property");
        let ignore_hidden_files = context
            .get_bool_property(&IGNORE_HIDDEN_FILES, None)?
            .expect("required property");

        Ok(GetFileRs {
            recursive,
            keep_source_file,
            input_directory,
            poll_interval,
            directory_listing: Mutex::new(DirectoryListing::new()),
            batch_size,
            min_size,
            max_size,
            min_age,
            max_age,
            ignore_hidden_files,
            metrics: Mutex::new(GetFileMetrics {
                accepted_files: 0,
                input_bytes: 0,
            }),
        })
    }
}

impl ConstTrigger for GetFileRs {
    fn trigger<PC, PS, L>(
        &self,
        _context: &mut PC,
        session: &mut PS,
        logger: &L,
    ) -> Result<OnTriggerResult, MinifiError>
    where
        PC: ProcessContext,
        PS: ProcessSession<FlowFile = PC::FlowFile>,
        L: Logger,
    {
        logger.trace(format!("on_trigger: {:?}", self).as_str());
        {
            let is_dir_empty_before_poll = self.is_listing_empty();
            logger.debug(
                format!(
                    "Listing is {} before polling directory",
                    is_dir_empty_before_poll
                )
                .as_str(),
            );
            if is_dir_empty_before_poll {
                if self.should_poll() {
                    self.perform_listing();
                }
            }
        }
        {
            let is_dir_empty_after_poll = self.is_listing_empty();
            logger.debug(
                format!(
                    "Listing is {} after polling directory",
                    is_dir_empty_after_poll
                )
                .as_str(),
            );
            if is_dir_empty_after_poll {
                return Ok(OnTriggerResult::Ok);
            }
        }

        let files = self.poll_listing(self.batch_size);
        for file in files {
            self.get_single_file(session, logger, file)?;
        }
        Ok(OnTriggerResult::Ok)
    }
}

impl CalculateMetrics for GetFileRs {
    fn calculate_metrics(&self) -> Vec<(String, f64)> {
        let metrics = self.metrics.lock().unwrap();
        vec![
            ("accepted_files".to_string(), metrics.accepted_files as f64),
            ("input_bytes".to_string(), metrics.input_bytes as f64),
        ]
    }
}

#[cfg(not(test))]
pub(crate) mod processor_definition;

mod output_attributes;
#[cfg(test)]
mod tests;

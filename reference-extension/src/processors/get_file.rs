use crate::processors::get_file::properties::{
    BATCH_SIZE, DIRECTORY, IGNORE_HIDDEN_FILES, KEEP_SOURCE_FILE, MAX_AGE, MAX_SIZE, MIN_AGE,
    MIN_SIZE, RECURSE,
};
use minifi_native::{
    Concurrent, ConcurrentOnTrigger, LogLevel, Logger, MinifiError, OnTriggerResult,
    ProcessContext, ProcessSession, Processor,
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

#[derive(Debug)]
pub(crate) struct GetFile<L: Logger> {
    logger: L,
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

impl<L: Logger> GetFile<L> {
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
        let walker = WalkDir::new(&self.input_directory);

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

        fn is_hidden(path: PathBuf) -> bool {
            // TODO(windows)
            path.file_name()
                .and_then(|f| f.to_str())
                .map_or(false, |f| f.starts_with('.'))
        }

        if self.ignore_hidden_files && is_hidden(dir_entry.path().to_path_buf()) {
            return Ok(false);
        }

        Ok(true)
    }

    fn get_single_file<PS>(&self, session: &mut PS, path: PathBuf)
    where
        PS: ProcessSession,
    {
        self.logger
            .info(format!("GetFile process {:?}", &path).as_str());
        let mut ff = session
            .create()
            .expect("Successful FlowFile creation is expected");

        if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
            session.set_attribute(&mut ff, "filename", file_name);
        } else {
            self.logger
                .warn(format!("Couldnt get filename of {:?}", path).as_str());
        }
        session.set_attribute(&mut ff, "absolute.path", path.to_string_lossy().trim());
        // TODO(relative path)

        let contents = std::fs::read_to_string(&path).expect("Failed to read file");
        session.write(&mut ff, contents.as_bytes()); // TODO(return value)
        if !self.keep_source_file {
            match std::fs::remove_file(&path) {
                Ok(_) => {}
                Err(err) => {
                    self.logger
                        .warn(format!("Failed to remove source file {:?}", err).as_str());
                }
            }
        }
        session.transfer(ff, relationships::SUCCESS.name);
    }
}

impl<L: Logger> Processor<L> for GetFile<L> {
    type Threading = Concurrent;

    fn new(logger: L) -> Self {
        Self {
            logger,
            recursive: true,
            keep_source_file: false,
            input_directory: PathBuf::new(),
            poll_interval: None,
            directory_listing: Mutex::new(DirectoryListing::new()),
            batch_size: 1,
            max_age: None,
            min_age: None,
            max_size: None,
            min_size: None,
            ignore_hidden_files: true,
            metrics: Mutex::new(GetFileMetrics {
                accepted_files: 0,
                input_bytes: 0,
            }),
        }
    }

    fn log(&self, log_level: LogLevel, message: &str) {
        self.logger.log(log_level, message);
    }

    fn on_schedule<P>(&mut self, context: &P) -> Result<(), MinifiError>
    where
        P: ProcessContext,
    {
        self.input_directory = context
            .get_property(&DIRECTORY, None)?
            .expect("Required property")
            .into();
        if !self.input_directory.is_dir() {
            return Err(MinifiError::ScheduleError(format!(
                "{:?} is not a valid directory",
                self.input_directory
            )));
        }

        self.recursive = context
            .get_bool_property(&RECURSE, None)?
            .expect("Required property");

        self.keep_source_file = context
            .get_bool_property(&KEEP_SOURCE_FILE, None)?
            .expect("Required property");

        self.poll_interval = context.get_duration_property(&properties::POLLING_INTERVAL, None)?;
        self.min_size = context.get_size_property(&MIN_SIZE, None)?;
        self.max_size = context.get_size_property(&MAX_SIZE, None)?;
        self.min_age = context.get_duration_property(&MIN_AGE, None)?;
        self.max_age = context.get_duration_property(&MAX_AGE, None)?;
        self.batch_size = context
            .get_u64_property(&BATCH_SIZE, None)?
            .expect("required property");
        self.ignore_hidden_files = context
            .get_bool_property(&IGNORE_HIDDEN_FILES, None)?
            .expect("required property");

        Ok(())
    }

    fn calculate_metrics(&self) -> Vec<(String, f64)> {
        let metrics = self.metrics.lock().unwrap();
        vec![
            ("accepted_files".to_string(), metrics.accepted_files as f64),
            ("input_bytes".to_string(), metrics.input_bytes as f64),
        ]
    }
}

impl<L: Logger> ConcurrentOnTrigger<L> for GetFile<L> {
    fn on_trigger<P, S>(
        &self,
        _context: &mut P,
        session: &mut S,
    ) -> Result<OnTriggerResult, MinifiError>
    where
        P: ProcessContext,
        S: ProcessSession,
    {
        self.logger
            .trace(format!("on_trigger: {:?}", self).as_str());
        {
            let is_dir_empty_before_poll = self.is_listing_empty();
            self.logger.debug(
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
            self.logger.debug(
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
            self.get_single_file(session, file);
        }
        Ok(OnTriggerResult::Ok)
    }
}

#[cfg(not(test))]
pub(crate) mod c_ffi_class_description;

#[cfg(test)]
mod tests;

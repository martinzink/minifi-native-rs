use crate::processors::get_file::properties::{
    BATCH_SIZE, DIRECTORY, KEEP_SOURCE_FILE, MAX_AGE, MAX_SIZE, MIN_AGE, MIN_SIZE, RECURSE,
};
use minifi_native::{LogLevel, Logger, MinifiError, ProcessContext, ProcessSession, Processor};
use std::collections::VecDeque;
use std::error;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use std::time::{Duration, Instant, SystemTime};
use walkdir::{DirEntry, WalkDir};

mod properties;
mod relationships;

#[derive(Debug)]
struct GetFile<L: Logger> {
    logger: L,
    recursive: bool,
    keep_source_file: bool,
    input_directory: PathBuf,
    poll_interval: Option<Duration>,
    directory_listing: VecDeque<PathBuf>,
    last_polling_time: Option<Instant>, // TODO(multithreading mutex)
    batch_size: u64,
    min_size: Option<u64>,
    max_size: Option<u64>,
    min_age: Option<Duration>,
    max_age: Option<Duration>,
    ignore_hidden_files: bool,
}

impl<L: Logger> GetFile<L> {
    fn is_listing_empty(&self) -> bool {
        // TODO(multithreading) mutex
        self.directory_listing.is_empty()
    }

    fn put_listing(&mut self, path: PathBuf) {
        // TODO(multithreading) mutex
        self.directory_listing.push_front(path);
    }

    fn poll_listing(&mut self, batch_size: u64) -> VecDeque<PathBuf> {
        // TODO(multithreading) mutex
        let mut res = VecDeque::new();
        for _ in 0..batch_size {
            if let Some(path) = self.directory_listing.pop_back() {
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
        if self.last_polling_time.is_none() {
            return true;
        }
        Instant::now() - self.last_polling_time.unwrap() > self.poll_interval.unwrap()
    }

    fn perform_listing(&mut self) {
        let walker = WalkDir::new(&self.input_directory);

        for entry in walker.into_iter().filter_map(Result::ok) {
            if self.entry_matches_criteria(&entry).unwrap_or(false) {
                self.put_listing(entry.into_path());
            }
        }
    }

    fn entry_matches_criteria(
        &mut self,
        dir_entry: &DirEntry,
    ) -> Result<bool, Box<dyn error::Error>> {
        let metadata = dir_entry.metadata()?;
        if !metadata.is_file() {
            return Ok(false);
        }
        let age = SystemTime::now().duration_since(metadata.modified()?)?;
        let size = metadata.size();

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

        // TODO(hidden files)

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
        session.write(&mut ff, contents.as_str()); // TODO(return value)
        if !self.keep_source_file {
            let _ = std::fs::remove_file(&path); // TODO(error handling)
        }
        session.transfer(ff, relationships::SUCCESS.name);
    }
}

impl<L: Logger> Processor<L> for GetFile<L> {
    fn new(logger: L) -> Self {
        Self {
            logger,
            recursive: true,
            keep_source_file: false,
            input_directory: PathBuf::new(),
            poll_interval: None,
            directory_listing: VecDeque::new(),
            last_polling_time: None, // TODO(multithreading mutex)
            batch_size: 1,
            max_age: None,
            min_age: None,
            max_size: None,
            min_size: None,
            ignore_hidden_files: true,
        }
    }

    fn on_trigger<P, S>(&mut self, context: &mut P, session: &mut S) -> Result<(), MinifiError>
    where
        P: ProcessContext,
        S: ProcessSession,
    {
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
                    self.last_polling_time = Some(Instant::now());
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
                context.yield_context();
                return Ok(());
            }
        }

        let files = self.poll_listing(self.batch_size);
        for file in files {
            self.get_single_file(session, file);
        }
        Ok(())
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

        Ok(())
    }

    fn log(&self, log_level: LogLevel, message: &str) {
        self.logger.log(log_level, message);
    }
}

#[cfg(not(test))]
mod register_ctor;

#[cfg(test)]
mod tests;

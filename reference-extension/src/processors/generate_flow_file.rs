use minifi_native::{
    Concurrent, ConcurrentOnTrigger, LogLevel, Logger, MinifiError, ProcessContext, ProcessSession,
    Processor,
};
use rand::Rng;
use rand::distr::Alphanumeric;
use std::cmp::PartialEq;

mod properties;
mod relationships;

#[derive(Debug, PartialEq)]
enum Mode {
    UniqueBytes,
    UniqueText,
    NotUniqueBytes,
    NotUniqueText,
    CustomText,
    Empty,
}

#[derive(Debug)]
struct GenerateFlowFile<L: Logger> {
    logger: L,
    mode: Mode,
    batch_size: u64,
    file_size: u64,
    data_generated_during_on_schedule: Vec<u8>,
}

impl<L: Logger> GenerateFlowFile<L> {
    fn is_unique(&self) -> bool {
        match self.mode {
            Mode::UniqueBytes => true,
            Mode::UniqueText => true,
            Mode::NotUniqueBytes => false,
            Mode::NotUniqueText => false,
            Mode::CustomText => false,
            Mode::Empty => false,
        }
    }

    fn is_text(&self) -> bool {
        match self.mode {
            Mode::UniqueBytes => false,
            Mode::UniqueText => true,
            Mode::NotUniqueBytes => false,
            Mode::NotUniqueText => true,
            Mode::CustomText => true,
            Mode::Empty => false,
        }
    }

    fn get_mode(is_unique: bool, is_text: bool, has_custom_text: bool, file_size: u64) -> Mode {
        if is_text && !is_unique && has_custom_text {
            return Mode::CustomText;
        }

        if file_size == 0 {
            return Mode::Empty;
        }

        match (is_unique, is_text) {
            (true, true) => Mode::UniqueText,
            (true, false) => Mode::UniqueBytes,
            (false, true) => Mode::NotUniqueText,
            (false, false) => Mode::NotUniqueBytes,
        }
    }

    fn generate_data(data: &mut [u8], text_data: bool) {
        let mut rng = rand::rng();

        if text_data {
            for byte in data.iter_mut() {
                *byte = rng.sample(Alphanumeric);
            }
        } else {
            rng.fill(data);
        }
    }
}

impl<L: Logger> Processor<L> for GenerateFlowFile<L> {
    type Threading = Concurrent;
    fn new(logger: L) -> Self {
        Self {
            logger,
            mode: Mode::Empty,
            batch_size: 1,
            file_size: 1024,
            data_generated_during_on_schedule: Vec::new(),
        }
    }

    fn log(&self, log_level: LogLevel, message: &str) {
        self.logger.log(log_level, message);
    }

    fn on_schedule<P: ProcessContext>(&mut self, context: &P) -> Result<(), MinifiError> {
        let is_unique = context
            .get_bool_property(&properties::UNIQUE_FLOW_FILES, None)?
            .expect("Required property");
        let is_text = context
            .get_property(&properties::DATA_FORMAT, None)?
            .expect("Required property")
            .as_str()
            == "Text";
        let has_custom_text = context
            .get_property(&properties::CUSTOM_TEXT, None)?
            .is_some();

        self.file_size = context
            .get_size_property(&properties::FILE_SIZE, None)?
            .expect("Required property");
        self.batch_size = context
            .get_u64_property(&properties::BATCH_SIZE, None)?
            .expect("Required property");

        self.mode = Self::get_mode(is_unique, is_text, has_custom_text, self.file_size);
        if self.mode == Mode::NotUniqueText || self.mode == Mode::NotUniqueBytes {
            self.data_generated_during_on_schedule = Vec::with_capacity(self.file_size as usize);
            Self::generate_data(&mut self.data_generated_during_on_schedule, is_text);
        }

        self.logger
            .trace(format!("GenerateFlowFile is configured as {:?}", self).as_str());
        Ok(())
    }
}

impl<L: Logger> ConcurrentOnTrigger<L> for GenerateFlowFile<L> {
    fn on_trigger<P: ProcessContext, S: ProcessSession>(
        &self,
        context: &mut P,
        session: &mut S,
    ) -> Result<(), MinifiError> {
        let non_unique_data_buffer: &[u8];
        let custom_text_for_batch: Option<String>;

        if self.mode == Mode::CustomText
            && let Some(custom_text) = context.get_property(&properties::CUSTOM_TEXT, None)?
        {
            custom_text_for_batch = Some(custom_text);
            non_unique_data_buffer = custom_text_for_batch.as_ref().unwrap().as_bytes();
        } else {
            non_unique_data_buffer = self.data_generated_during_on_schedule.as_slice();
        }

        for _ in 0..self.batch_size {
            let mut ff = session.create()?;
            if self.file_size != 0 {
                if self.is_unique() {
                    let mut unique_data: Vec<u8> = Vec::with_capacity(self.file_size as usize);
                    Self::generate_data(&mut unique_data, self.is_text());
                    session.write(&mut ff, unique_data.as_slice());
                } else {
                    session.write(&mut ff, non_unique_data_buffer);
                }
            }
            session.transfer(ff, relationships::SUCCESS.name);
        }
        Ok(())
    }
}

#[cfg(not(test))]
mod register_ctor;

#[cfg(test)]
mod tests;

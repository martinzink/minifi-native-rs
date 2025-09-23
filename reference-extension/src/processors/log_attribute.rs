use crate::processors::log_attribute::properties::{FLOW_FILES_TO_LOG, LOG_LEVEL, LOG_PAYLOAD};
use minifi_native::{
    ConcurrentOnTrigger, LogLevel, Logger, MinifiError, ProcessContext, ProcessSession, Processor,
    Property,
};

mod properties;
mod relationships;

#[derive(Debug)]
struct LogAttribute<L: Logger> {
    logger: L,
    log_level: LogLevel,
    attributes_to_log: Option<Vec<String>>,
    attributes_to_ignore: Option<Vec<String>>,
    log_payload: bool,
    flow_files_to_log: usize,
    dash_line: String,
    hex_encode_payload: bool,
}

impl<L: Logger> LogAttribute<L> {
    fn generate_log_message<PS>(&self, session: &mut PS, flow_file: &mut PS::FlowFile) -> String
    where
        PS: ProcessSession,
    {
        let mut log_msg = String::with_capacity(1024);
        log_msg.push_str("Logging for flow file\n");
        log_msg.push_str(self.dash_line.as_str());

        log_msg.push_str("\nFlowFile Attributes Map Content");
        session.on_attributes(flow_file, |key, value| {
            if let Some(attributes_to_ignore) = &self.attributes_to_ignore {
                if attributes_to_ignore.iter().any(|ign| ign == key) {
                    return;
                }
            }
            if let Some(attributes_to_log) = &self.attributes_to_log {
                if !attributes_to_log.iter().any(|ign| ign == key) {
                    return;
                }
            }
            log_msg.push_str(format!("\nkey:{} value:{}", &key, &value).as_str());
        });
        if self.log_payload {
            log_msg.push_str("\nPayload:\n");
            let flow_file_payload = session.read(flow_file).unwrap();
            if self.hex_encode_payload {
                log_msg.push_str(&hex::encode(flow_file_payload));
            } else {
                log_msg.push_str(String::from_utf8(flow_file_payload).unwrap_or(String::new()).as_str());  // TODO(error handling)
            }
        }
        log_msg.push_str("\n");
        log_msg.push_str(self.dash_line.as_str());
        log_msg
    }
}

impl<L: Logger> ConcurrentOnTrigger<L> for LogAttribute<L> {
    fn on_trigger<P, S>(&self, _context: &mut P, session: &mut S) -> Result<(), MinifiError>
    where
        P: ProcessContext,
        S: ProcessSession,
    {
        self.logger.trace(
            format!(
                "enter log attribute, attempting to retrieve {} flow files",
                self.flow_files_to_log
            )
            .as_str(),
        );
        let max_flow_files_to_process = if self.flow_files_to_log == 0 {
            usize::MAX
        } else {
            self.flow_files_to_log
        };
        let mut flow_files_processed = 0usize;
        for _ in 0..max_flow_files_to_process {
            if let Some(mut flow_file) = session.get() {
                let log_msg = self.generate_log_message(session, &mut flow_file);
                self.logger.log(self.log_level, log_msg.as_str());
                session.transfer(flow_file, relationships::SUCCESS.name);
                flow_files_processed += 1;
            } else {
                break;
            }
        }
        self.logger
            .debug(format!("Logged {} flow files", flow_files_processed).as_str());

        Ok(())
    }
}

impl<L: Logger> Processor<L> for LogAttribute<L> {
    type Threading = minifi_native::Concurrent;
    fn new(logger: L) -> Self {
        Self {
            logger,
            log_level: LogLevel::Info,
            attributes_to_log: None,
            attributes_to_ignore: None,
            log_payload: false,
            flow_files_to_log: 1,
            dash_line: String::new(),
            hex_encode_payload: false,
        }
    }

    fn log(&self, log_level: LogLevel, message: &str) {
        self.logger.log(log_level, message);
    }

    fn on_schedule<P>(&mut self, context: &P) -> Result<(), MinifiError>
    where
        P: ProcessContext,
    {
        self.log_level = context
            .get_property(&LOG_LEVEL, None)?
            .expect("required property")
            .parse::<LogLevel>()?;

        self.log_payload = context
            .get_bool_property(&LOG_PAYLOAD, None)?
            .expect("required property");

        self.flow_files_to_log = context
            .get_property(&FLOW_FILES_TO_LOG, None)?
            .expect("required property")
            .parse::<usize>()?;

        fn get_csv_property<P: ProcessContext>(
            context: &P,
            property: &Property,
        ) -> Result<Option<Vec<String>>, MinifiError> {
            Ok(context
                .get_property(property, None)?
                .and_then(|s| Some(s.split(",").map(|s| s.to_string()).collect::<Vec<String>>())))
        }

        self.attributes_to_log = get_csv_property(context, &properties::ATTRIBUTES_TO_LOG)?;
        self.attributes_to_ignore = get_csv_property(context, &properties::ATTRIBUTES_TO_IGNORE)?;

        self.dash_line = format!(
            "{:-^50}",
            context
                .get_property(&properties::LOG_PREFIX, None)?
                .unwrap_or(String::new())
        );

        self.hex_encode_payload = context
            .get_bool_property(&properties::HEX_ENCODE_PAYLOAD, None)?
            .expect("required property");

        Ok(())
    }
}

#[cfg(not(test))]
mod register_ctor;

#[cfg(test)]
mod tests;

use minifi_native::{MinifiError, LogLevel, Logger, ProcessContext, ProcessSession, Processor, Property};
use crate::processors::log_attribute::properties::{FLOW_FILES_TO_LOG, LOG_LEVEL, LOG_PAYLOAD};

mod relationships;
mod properties;


#[derive(Debug)]
struct LogAttribute<L: Logger> {
    logger: L,
    log_level: LogLevel,
    attributes_to_log: Option<Vec<String>>,
    attributes_to_ignore: Option<Vec<String>>,
    log_payload: bool,
    flow_files_to_log_: usize,
    dash_line_: String,
}

impl<L: Logger> LogAttribute<L> {
    fn generate_log_message<PS>(&self, session: &mut PS, flow_file: &mut PS::FlowFile) -> String
    where PS: ProcessSession,
    {
        let mut log_msg = String::with_capacity(1024);
        log_msg.push_str("Logging for flow file\n");
        log_msg.push_str(self.dash_line_.as_str());

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
            log_msg.push_str(session.read_as_string(flow_file).unwrap_or(String::new()).as_str());
        }
        log_msg.push_str("\n");
        log_msg.push_str(self.dash_line_.as_str());
        log_msg
    }
}

impl<L: Logger> Processor<L> for LogAttribute<L> {
    fn new(logger: L) -> Self {
        Self {
            logger,
            log_level: LogLevel::Info,
            attributes_to_log: None,
            attributes_to_ignore: None,
            log_payload: false,
            flow_files_to_log_: 1,
            dash_line_: String::new()
        }
    }

    fn on_trigger<P, S>(&mut self, _context: &P, session: &mut S) -> Result<(), MinifiError>
    where
        P: ProcessContext,
        S: ProcessSession,
    {
        self.logger.trace(format!("enter log attribute, attempting to retrieve {} flow files", self.flow_files_to_log_).as_str());
        let max_flow_files_to_process = if self.flow_files_to_log_ == 0 { usize::MAX } else { self.flow_files_to_log_ };
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
        self.logger.debug(format!("Logged {} flow files", flow_files_processed).as_str());

        Ok(())
    }


    fn on_schedule<P>(&mut self, context: &P) -> Result<(), MinifiError>
    where
        P: ProcessContext,
    {
        self.log_level = context
            .get_property(&LOG_LEVEL, None)
            .ok_or(MinifiError::MissingRequiredProperty(LOG_LEVEL.name))?
            .parse::<LogLevel>()?;

        self.log_payload = context
            .get_property(&LOG_PAYLOAD, None)
            .ok_or(MinifiError::MissingRequiredProperty(LOG_PAYLOAD.name))?
            .parse::<bool>()?;

        self.flow_files_to_log_ = context
            .get_property(&FLOW_FILES_TO_LOG, None)
            .ok_or(MinifiError::MissingRequiredProperty(FLOW_FILES_TO_LOG.name))?
            .parse::<usize>()?;

        fn parse_comma_separated<P: ProcessContext>(context: &P, property: &Property) -> Option<Vec<String>> {
            context
                .get_property(property, None)
                .and_then(|s| Some(s.split(",").map(|s| s.to_string()).collect::<Vec<String>>()))
        }

        self.attributes_to_log = parse_comma_separated(context, &properties::ATTRIBUTES_TO_LOG);
        self.attributes_to_ignore = parse_comma_separated(context, &properties::ATTRIBUTES_TO_IGNORE);

        self.dash_line_ = format!("{:-^50}", context.get_property(&properties::LOG_PREFIX, None).unwrap_or(String::new()));

        Ok(())
    }

    fn log(&mut self, log_level: LogLevel, message: &str) {
        self.logger.log(log_level, message);
    }
}

#[cfg(not(test))]
mod register_ctor;

#[cfg(test)]
mod tests;

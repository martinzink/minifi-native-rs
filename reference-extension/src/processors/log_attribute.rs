use crate::processors::log_attribute::properties::{FLOW_FILES_TO_LOG, LOG_LEVEL, LOG_PAYLOAD};
use minifi_native::{
    ConcurrentOnTrigger, DefaultLogger, LogLevel, Logger, MinifiError, OnTriggerResult,
    ProcessContext, ProcessSession, RawProcessor, Property,
};

mod properties;
mod relationships;

#[derive(Debug)]
struct ScheduledMembers {
    log_level: LogLevel,
    attributes_to_log: Option<Vec<String>>,
    attributes_to_ignore: Option<Vec<String>>,
    log_payload: bool,
    flow_files_to_log: usize,
    dash_line: String,
    hex_encode_payload: bool,
}

#[derive(Debug)]
pub(crate) struct LogAttribute {
    logger: DefaultLogger,
    scheduled_members: Option<ScheduledMembers>,
}

impl LogAttribute {
    fn generate_log_message<PS>(&self, session: &mut PS, flow_file: &mut PS::FlowFile) -> String
    where
        PS: ProcessSession,
    {
        let log_attribute = self
            .scheduled_members
            .as_ref()
            .expect("on_schedule should create GetFileImpl");
        let mut log_msg = String::with_capacity(1024);
        log_msg.push_str("Logging for flow file\n");
        log_msg.push_str(log_attribute.dash_line.as_str());

        log_msg.push_str("\nFlowFile Attributes Map Content");
        session.on_attributes(flow_file, |key, value| {
            if let Some(attributes_to_ignore) = &log_attribute.attributes_to_ignore {
                if attributes_to_ignore.iter().any(|ign| ign == key) {
                    return;
                }
            }
            if let Some(attributes_to_log) = &log_attribute.attributes_to_log {
                if !attributes_to_log.iter().any(|ign| ign == key) {
                    return;
                }
            }
            log_msg.push_str(format!("\nkey:{} value:{}", &key, &value).as_str());
        });
        if log_attribute.log_payload {
            log_msg.push_str("\nPayload:\n");
            if let Some(flow_file_payload) = session.read(flow_file) {
                if log_attribute.hex_encode_payload {
                    log_msg.push_str(&hex::encode(flow_file_payload));
                } else {
                    log_msg.push_str(
                        String::from_utf8(flow_file_payload)
                            .unwrap_or(String::new())
                            .as_str(),
                    ); // TODO(error handling)
                }
            }
        }
        log_msg.push_str("\n");
        log_msg.push_str(log_attribute.dash_line.as_str());
        log_msg
    }
}

impl ConcurrentOnTrigger for LogAttribute {
    fn on_trigger<P, S>(
        &self,
        _context: &mut P,
        session: &mut S,
    ) -> Result<OnTriggerResult, MinifiError>
    where
        P: ProcessContext,
        S: ProcessSession,
    {
        let log_attribute = self
            .scheduled_members
            .as_ref()
            .expect("on_schedule should create GetFileImpl");
        self.logger.trace(
            format!(
                "enter log attribute, attempting to retrieve {} flow files",
                log_attribute.flow_files_to_log
            )
            .as_str(),
        );
        let max_flow_files_to_process = if log_attribute.flow_files_to_log == 0 {
            usize::MAX
        } else {
            log_attribute.flow_files_to_log
        };
        let mut flow_files_processed = 0usize;
        for _ in 0..max_flow_files_to_process {
            if let Some(mut flow_file) = session.get() {
                let log_msg = self.generate_log_message(session, &mut flow_file);
                self.logger.log(log_attribute.log_level, log_msg.as_str());
                session.transfer(flow_file, relationships::SUCCESS.name);
                flow_files_processed += 1;
            } else {
                break;
            }
        }
        self.logger
            .debug(format!("Logged {} flow files", flow_files_processed).as_str());

        Ok(OnTriggerResult::Ok)
    }
}

impl RawProcessor for LogAttribute {
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

    fn on_schedule<P>(&mut self, context: &P) -> Result<(), MinifiError>
    where
        P: ProcessContext,
    {
        let log_level = context
            .get_property(&LOG_LEVEL, None)?
            .expect("required property")
            .parse::<LogLevel>()?;

        let log_payload = context
            .get_bool_property(&LOG_PAYLOAD, None)?
            .expect("required property");

        let flow_files_to_log = context
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

        let attributes_to_log = get_csv_property(context, &properties::ATTRIBUTES_TO_LOG)?;
        let attributes_to_ignore = get_csv_property(context, &properties::ATTRIBUTES_TO_IGNORE)?;

        let dash_line = format!(
            "{:-^50}",
            context
                .get_property(&properties::LOG_PREFIX, None)?
                .unwrap_or(String::new())
        );

        let hex_encode_payload = context
            .get_bool_property(&properties::HEX_ENCODE_PAYLOAD, None)?
            .expect("required property");

        self.scheduled_members = Some(ScheduledMembers {
            log_level,
            attributes_to_log,
            attributes_to_ignore,
            log_payload,
            flow_files_to_log,
            dash_line,
            hex_encode_payload,
        });
        Ok(())
    }
}

#[cfg(not(test))]
pub(crate) mod processor_definition;

#[cfg(test)]
mod tests;

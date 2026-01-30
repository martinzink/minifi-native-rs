use crate::processors::log_attribute::properties::{FLOW_FILES_TO_LOG, LOG_LEVEL, LOG_PAYLOAD};
use minifi_native::{DefaultLogger, LogLevel, Logger, MinifiError, OnTriggerResult, ProcessContext, ProcessSession, Property, ConstTriggerable, Schedulable, MetricsProvider};

mod properties;
mod relationships;

#[derive(Debug)]
pub(crate) struct LogAttribute {
    log_level: LogLevel,
    attributes_to_log: Option<Vec<String>>,
    attributes_to_ignore: Option<Vec<String>>,
    log_payload: bool,
    flow_files_to_log: usize,
    dash_line: String,
    hex_encode_payload: bool,
}

impl LogAttribute {
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
            if let Some(flow_file_payload) = session.read(flow_file) {
                if self.hex_encode_payload {
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
        log_msg.push_str(self.dash_line.as_str());
        log_msg
    }
}

impl ConstTriggerable for LogAttribute {
    fn trigger<PC, PS>(&self, _context: &mut PC, session: &mut PS, logger: &DefaultLogger) -> Result<OnTriggerResult, MinifiError>
    where
        PC: ProcessContext,
        PS: ProcessSession<FlowFile=PC::FlowFile>
    {
        logger.trace(
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
                logger.log(self.log_level, log_msg.as_str());
                session.transfer(flow_file, relationships::SUCCESS.name);
                flow_files_processed += 1;
            } else {
                break;
            }
        }
        logger
            .debug(format!("Logged {} flow files", flow_files_processed).as_str());

        Ok(OnTriggerResult::Ok)
    }
}

impl Schedulable for LogAttribute {
    fn schedule<P: ProcessContext>(context: &P, _logger: &DefaultLogger) -> Result<Self, MinifiError>
    where
        Self: Sized,
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

        Ok(LogAttribute {
            log_level,
            attributes_to_log,
            attributes_to_ignore,
            log_payload,
            flow_files_to_log,
            dash_line,
            hex_encode_payload,
        })
    }
}

impl MetricsProvider for LogAttribute {}

#[cfg(not(test))]
pub(crate) mod processor_definition;

#[cfg(test)]
mod tests;

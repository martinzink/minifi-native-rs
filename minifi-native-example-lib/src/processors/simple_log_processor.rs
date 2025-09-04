use minifi_native::ProcessorInputRequirement::Allowed;
use minifi_native::{
    CffiLogger, LogLevel, Logger, ProcessContext, ProcessSession, Processor,
    ProcessorDefinition, Property, Relationship, StandardPropertyValidator,
};
use strum::VariantNames;

#[derive(Debug)]
struct SimpleLogProcessor<L: Logger> {
    logger: L,
    log_level: LogLevel,
}

const SUCCESS_RELATIONSHIP: Relationship = Relationship {
    name: "success",
    description: "FlowFiles are transferred here after logging",
};

const LOG_LEVEL: Property = Property {
    name: "Log Level",
    description: "The level of logging.",
    is_required: true,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: Some("info"),
    validator: StandardPropertyValidator::AlwaysValidValidator,
    allowed_values: &LogLevel::VARIANTS,
    allowed_types: &[],
};

impl<L: Logger> Processor<L> for SimpleLogProcessor<L> {
    fn new(logger: L) -> Self {
        Self {
            logger,
            log_level: LogLevel::Info,
        }
    }

    fn on_trigger<P, S>(&mut self, _context: &P, session: &mut S)
    where
        P: ProcessContext,
        S: ProcessSession,
    {
        self.logger
            .trace(format!("on_trigger entry {:?}", self).as_str());

        if let Some(input_ff) = session.get() {
            self.logger.trace(format!("Got flowfile").as_str());
            if let Some(content) = session.read(&input_ff) {
                self.logger.log(self.log_level, content.as_str());
            }
            session.transfer(input_ff, SUCCESS_RELATIONSHIP.name);
        }

        self.logger
            .trace(format!("on_trigger exit {:?}", self).as_str());
    }

    fn on_schedule<P>(&mut self, context: &P)
    where
        P: ProcessContext,
    {
        self.logger
            .trace(format!("on_schedule entry {:?}", self).as_str());

        self.log_level = context
            .get_property(LOG_LEVEL.name, None)
            .and_then(|s| s.parse::<LogLevel>().ok())
            .unwrap_or(LogLevel::Info);

        self.logger
            .trace(format!("on_schedule exit {:?}", self).as_str());
    }
}

#[cfg_attr(test, allow(dead_code))]
fn simple_log_processor_definition() -> ProcessorDefinition<SimpleLogProcessor<CffiLogger>> {
    let mut simple_log_processor_definition =
        ProcessorDefinition::<SimpleLogProcessor<CffiLogger>>::new(
            "rust_extension",
            "mzink.processors.rust.SimpleLogProcessor",
            "A rust processor that tests the upcoming C API, trying out most of the features",
        );

    simple_log_processor_definition.is_single_threaded = true;
    simple_log_processor_definition.input_requirement = Allowed;
    simple_log_processor_definition.supports_dynamic_properties = false;
    simple_log_processor_definition.supports_dynamic_relationships = false;
    simple_log_processor_definition.relationships = vec![SUCCESS_RELATIONSHIP];
    simple_log_processor_definition.properties = vec![LOG_LEVEL];
    simple_log_processor_definition
}

#[cfg(not(test))]
#[ctor::ctor]
#[unsafe(no_mangle)]
fn register_simple_log_processor() {
    simple_log_processor_definition().register_class();
}

#[cfg(test)]
mod tests;

use minifi_native::ProcessorInputRequirement::Forbidden;
use minifi_native::{CffiLogger, MinifiError, Logger, ProcessContext, ProcessSession, Processor, ProcessorDefinition, LogLevel};

mod relationships;
mod properties;

#[derive(Debug)]
struct SimpleSourceProcessor<L: Logger> {
    logger: L,
    content: String,
}

impl<L: Logger> Processor<L> for SimpleSourceProcessor<L> {
    fn new(logger: L) -> Self {
        Self {
            logger,
            content: String::new(),
        }
    }

    fn on_trigger<P, S>(&mut self, _context: &mut P, session: &mut S) -> Result<(), MinifiError>
    where
        P: ProcessContext,
        S: ProcessSession,
    {
        self.logger
            .trace(format!("on_trigger exit {:?}", self).as_str());

        if let Some(mut new_ff) = session.create() {
            self.logger
                .info("Created new flowfile".to_string().as_str());
            session.set_attribute(&mut new_ff, "source", "SimpleSourceProcessor");
            session.write(&mut new_ff, self.content.as_str());
            session.transfer(new_ff, relationships::SUCCESS.name);
        }

        self.logger
            .trace(format!("on_trigger exit {:?}", self).as_str());
        Ok(())
    }

    fn on_schedule<P>(&mut self, context: &P) -> Result<(), MinifiError>
    where
        P: ProcessContext,
    {
        self.logger
            .trace(format!("on_schedule entry {:?}", self).as_str());

        let shouting = context
            .get_property(&properties::SHOUT, None)?
            .and_then(|s| s.parse::<bool>().ok())
            .unwrap_or(false);

        self.content = context
            .get_property(&properties::CONTENT, None)?
            .unwrap_or("Default content".to_string());
        if shouting {
            self.content = self.content.to_uppercase();
        }
        self.logger
            .trace(format!("on_schedule exit {:?}", self).as_str());
        Ok(())
    }

    fn log(&self, log_level: LogLevel, message: &str) {
        self.logger.log(log_level, message);
    }
}

#[cfg_attr(test, allow(dead_code))]
fn create_simple_source_processor_definition()
-> ProcessorDefinition<SimpleSourceProcessor<CffiLogger>> {
    let mut simple_source_processor_definition =
        ProcessorDefinition::<SimpleSourceProcessor<CffiLogger>>::new(
            "rust_reference_extension",
            "mzink::processors::rust::SimpleSourceProcessor",
            "A rust processor that acts as a source.",
        );

    simple_source_processor_definition.is_single_threaded = true;
    simple_source_processor_definition.input_requirement = Forbidden;
    simple_source_processor_definition.supports_dynamic_properties = false;
    simple_source_processor_definition.supports_dynamic_relationships = false;
    simple_source_processor_definition.relationships = &[relationships::SUCCESS];
    simple_source_processor_definition.properties = &[properties::CONTENT, properties::SHOUT];

    simple_source_processor_definition
}

#[cfg(not(test))]
#[ctor::ctor]
#[unsafe(no_mangle)]
fn register_simple_source_processor() {
    create_simple_source_processor_definition().register_class();
}

#[cfg(test)]
mod tests;

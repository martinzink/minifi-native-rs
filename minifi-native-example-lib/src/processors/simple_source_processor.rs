use minifi_native::ProcessorInputRequirement::Forbidden;
use minifi_native::{
    CffiLogger, Logger, ProcessContext, ProcessSession, ProcessSessionFactory, Processor,
    ProcessorDefinition, Property, Relationship, StandardPropertyValidator,
};

#[derive(Debug)]
struct SimpleSourceProcessor<L: Logger> {
    logger: L,
    content: String,
}

const SUCCESS_RELATIONSHIP: Relationship = Relationship {
    name: "success",
    description: "FlowFiles are transferred here after logging",
};

const CONTENT_PROPERTY: Property = Property {
    name: "Content",
    description: "What to write to the flowfile.",
    is_required: false,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: Some("Something default to write"),
    validator: StandardPropertyValidator::AlwaysValidValidator,
    allowed_values: &[],
    allowed_types: &[],
};

const SHOUT_PROPERTY: Property = Property {
    name: "Shouting",
    description: "do you want to shout?",
    is_required: true,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: Some("false"),
    validator: StandardPropertyValidator::BoolValidator,
    allowed_values: &[],
    allowed_types: &[],
};

impl<L: Logger> Processor<L> for SimpleSourceProcessor<L> {
    fn new(logger: L) -> Self {
        Self {
            logger,
            content: String::new(),
        }
    }

    fn on_trigger<P, S>(&mut self, _context: &P, session: &mut S)
    where
        P: ProcessContext,
        S: ProcessSession,
    {
        self.logger
            .trace(format!("on_trigger exit {:?}", self).as_str());

        if let Some(mut new_ff) = session.create() {
            self.logger.info(format!("Created new flowfile").as_str());
            session.write(&mut new_ff, self.content.as_str());
            session.transfer(new_ff, SUCCESS_RELATIONSHIP.name);
        }

        self.logger
            .trace(format!("on_trigger exit {:?}", self).as_str());
    }

    fn on_schedule<P, F>(&mut self, context: &P, _session_factory: &mut F)
    where
        P: ProcessContext,
        F: ProcessSessionFactory,
    {
        self.logger
            .trace(format!("on_schedule entry {:?}", self).as_str());

        let shouting = context
            .get_property(SHOUT_PROPERTY.name, None)
            .and_then(|s| s.parse::<bool>().ok())
            .unwrap_or(false);

        self.content = context
            .get_property(CONTENT_PROPERTY.name, None)
            .unwrap_or("Default content".to_string());
        if shouting {
            self.content = self.content.to_uppercase();
        }
        self.logger
            .trace(format!("on_schedule exit {:?}", self).as_str());
    }
}

#[cfg_attr(test, allow(dead_code))]
fn create_simple_source_processor_definition()
-> ProcessorDefinition<SimpleSourceProcessor<CffiLogger>> {
    let mut simple_source_processor_definition =
        ProcessorDefinition::<SimpleSourceProcessor<CffiLogger>>::new(
            "rust_extension",
            "mzink.processors.rust.SimpleSourceProcessor",
            "A rust processor that acts as a source.",
        );

    simple_source_processor_definition.is_single_threaded = true;
    simple_source_processor_definition.input_requirement = Forbidden;
    simple_source_processor_definition.supports_dynamic_properties = false;
    simple_source_processor_definition.supports_dynamic_relationships = false;
    simple_source_processor_definition.relationships = vec![SUCCESS_RELATIONSHIP];
    simple_source_processor_definition.properties = vec![CONTENT_PROPERTY, SHOUT_PROPERTY];

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

mod properties;

use crate::controller_services::lorem_ipsum_controller_service::LoremIpsumControllerService;
use crate::processors::lorem_ipsum_cs_user::properties::CONTROLLER_SERVICE;
use crate::processors::lorem_ipsum_cs_user::relationships::SUCCESS;
use minifi_native::macros::{ComponentIdentifier, DefaultMetrics};
use minifi_native::{
    Content, FlowFileSource, GeneratedFlowFile, Logger, MinifiError, ProcessContext, Schedule,
};
use std::collections::HashMap;
use strum_macros::{Display, EnumString, IntoStaticStr, VariantNames};

#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString, VariantNames, IntoStaticStr)]
#[strum(serialize_all = "PascalCase", const_into_str)]
enum WriteMethod {
    Buffer,
    Stream,
}

#[derive(Debug, ComponentIdentifier, DefaultMetrics)]
pub(crate) struct LoremIpsumCSUser {
    write_method: WriteMethod,
}

impl Schedule for LoremIpsumCSUser {
    fn schedule<P: ProcessContext, L: Logger>(context: &P, _logger: &L) -> Result<Self, MinifiError>
    where
        Self: Sized,
    {
        let write_method = context
            .get_property(&properties::WRITE_METHOD, None)?
            .expect("required property")
            .parse::<WriteMethod>()?;
        Ok(Self { write_method })
    }
}

impl FlowFileSource for LoremIpsumCSUser {
    fn generate<'a, Context: ProcessContext, LoggerImpl: Logger>(
        &self,
        context: &'a mut Context,
        logger: &LoggerImpl,
    ) -> Result<Option<GeneratedFlowFile<'a>>, MinifiError> {
        logger.trace(&format!("generate call {:?}", self));
        let controller_service = context
            .get_controller_service::<LoremIpsumControllerService>(&CONTROLLER_SERVICE)?
            .ok_or(MinifiError::MissingRequiredProperty(
                "A valid usable controller service is required",
            ))?;
        match self.write_method {
            WriteMethod::Buffer => {
                let generated_flow_file = GeneratedFlowFile::new(
                    &SUCCESS,
                    Some(Content::from(controller_service.data.clone())),
                    HashMap::new(),
                );
                Ok(Some(generated_flow_file))
            }
            WriteMethod::Stream => {
                let reader = controller_service.data.as_bytes();
                let content = Content::Stream(Box::new(reader));
                let generated_flow_file =
                    GeneratedFlowFile::new(&SUCCESS, Some(content), HashMap::new());
                Ok(Some(generated_flow_file))
            }
        }
    }
}

#[cfg(not(test))]
pub(crate) mod processor_definition;

mod relationships;
#[cfg(test)]
mod tests;

use crate::api::property::{GetControllerService, GetProperty};
use crate::{
    ComponentIdentifier, EnableControllerService, MinifiError, ProcessContext, ProcessSession,
    Property,
};

pub trait GetAttribute {
    fn get_attribute(&self, name: &str) -> Result<String, MinifiError>;
}

pub struct ContextSessionWithFlowFile<'a, PC, PS>
where
    PC: ProcessContext,
    PS: ProcessSession<FlowFile = PC::FlowFile>,
{
    context: &'a PC,
    session: &'a PS,
    flow_file: Option<&'a PC::FlowFile>,
}

impl<'a, PC, PS> ContextSessionWithFlowFile<'a, PC, PS>
where
    PC: ProcessContext,
    PS: ProcessSession<FlowFile = PC::FlowFile>,
{
    pub(crate) fn new(
        context: &'a PC,
        session: &'a PS,
        flow_file: Option<&'a PC::FlowFile>,
    ) -> Self {
        Self {
            context,
            session,
            flow_file,
        }
    }
}
impl<'a, PC, PS> GetProperty for ContextSessionWithFlowFile<'a, PC, PS>
where
    PC: ProcessContext,
    PS: ProcessSession<FlowFile = PC::FlowFile>,
{
    fn get_property(&self, property: &Property) -> Result<Option<String>, MinifiError> {
        self.context.get_property(property, self.flow_file)
    }
}

impl<'a, PC, PS> GetControllerService for ContextSessionWithFlowFile<'a, PC, PS>
where
    PC: ProcessContext,
    PS: ProcessSession<FlowFile = PC::FlowFile>,
{
    fn get_controller_service<Cs>(&self, property: &Property) -> Result<Option<&Cs>, MinifiError>
    where
        Cs: EnableControllerService + ComponentIdentifier + 'static,
    {
        self.context.get_controller_service(property)
    }
}

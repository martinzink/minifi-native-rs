use crate::{
    ComponentIdentifier, EnableControllerService, MinifiError, ProcessContext, ProcessSession,
    Property,
};

struct SimpleContext<'a, PC, PS>
where
    PC: ProcessContext,
    PS: ProcessSession<FlowFile = PC::FlowFile>,
{
    context: &'a PC,
    session: &'a PS,
    flow_file: Option<&'a PC::FlowFile>,
}

impl<'a, PC, PS> SimpleContext<'a, PC, PS>
where
    PC: ProcessContext,
    PS: ProcessSession<FlowFile = PC::FlowFile>,
{
    fn get_property(&self, property: &Property) -> Result<Option<String>, MinifiError> {
        self.context.get_property(property, self.flow_file)
    }

    fn get_controller_service<Cs>(&self, property: &Property) -> Result<Option<&Cs>, MinifiError>
    where
        Cs: EnableControllerService + ComponentIdentifier + 'static,
    {
        self.context.get_controller_service(property)
    }

    fn get_attribute(&self, attr_name: &str) -> Option<String> {
        if let Some(ff) = self.flow_file {
            self.session.get_attribute(ff, attr_name)
        } else {
            None
        }
    }
}

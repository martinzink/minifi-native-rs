use crate::{MinifiError, Property};

pub trait ControllerServiceContext {
    fn get_property(&self, property: &Property) -> Result<Option<String>, MinifiError>;
}

use crate::StandardPropertyValidator::{DataSizeValidator, U64Validator};
use crate::{MinifiError, Property};
use std::str::FromStr;

pub trait ControllerServiceContext {
    fn get_property(&self, property: &Property) -> Result<Option<String>, MinifiError>;

    fn get_u64_property(&self, property: &Property) -> Result<Option<u64>, MinifiError> {
        if property.validator != U64Validator {
            return Err(MinifiError::InvalidValidator);
        }
        if let Some(property_val) = self.get_property(property)? {
            Ok(Some(u64::from_str(&property_val)?))
        } else {
            Ok(None)
        }
    }

    fn get_size_property(&self, property: &Property) -> Result<Option<u64>, MinifiError> {
        if property.validator != DataSizeValidator {
            return Err(MinifiError::InvalidValidator);
        }
        if let Some(property_val) = self.get_property(property)? {
            Ok(Some(byte_unit::Byte::from_str(&property_val)?.as_u64()))
        } else {
            Ok(None)
        }
    }
}

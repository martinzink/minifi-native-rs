use minifi_native_sys::{MinifiStandardPropertyValidator, MinifiStandardPropertyValidator_MINIFI_ALWAYS_VALID_VALIDATOR, MinifiStandardPropertyValidator_MINIFI_BOOLEAN_VALIDATOR, MinifiStandardPropertyValidator_MINIFI_DATA_SIZE_VALIDATOR, MinifiStandardPropertyValidator_MINIFI_INTEGER_VALIDATOR, MinifiStandardPropertyValidator_MINIFI_NON_BLANK_VALIDATOR, MinifiStandardPropertyValidator_MINIFI_PORT_VALIDATOR, MinifiStandardPropertyValidator_MINIFI_TIME_PERIOD_VALIDATOR, MinifiStandardPropertyValidator_MINIFI_UNSIGNED_INTEGER_VALIDATOR};

pub enum StandardPropertyValidator {
    AlwaysValidValidator,
    NonBlankValidator,
    TimePeriodValidator,
    BoolValidator,
    I64Validator,
    U64Validator,
    DataSizeValidator,
    PortValidator,
}

impl StandardPropertyValidator {
    pub(crate) fn getMinifiPropertyValidatorValue(&self) -> MinifiStandardPropertyValidator {
        match self {
            StandardPropertyValidator::AlwaysValidValidator => MinifiStandardPropertyValidator_MINIFI_ALWAYS_VALID_VALIDATOR,
            StandardPropertyValidator::NonBlankValidator => MinifiStandardPropertyValidator_MINIFI_NON_BLANK_VALIDATOR,
            StandardPropertyValidator::TimePeriodValidator => MinifiStandardPropertyValidator_MINIFI_TIME_PERIOD_VALIDATOR,
            StandardPropertyValidator::BoolValidator => MinifiStandardPropertyValidator_MINIFI_BOOLEAN_VALIDATOR,
            StandardPropertyValidator::I64Validator => MinifiStandardPropertyValidator_MINIFI_INTEGER_VALIDATOR,
            StandardPropertyValidator::U64Validator => MinifiStandardPropertyValidator_MINIFI_UNSIGNED_INTEGER_VALIDATOR,
            StandardPropertyValidator::DataSizeValidator => MinifiStandardPropertyValidator_MINIFI_DATA_SIZE_VALIDATOR,
            StandardPropertyValidator::PortValidator => MinifiStandardPropertyValidator_MINIFI_PORT_VALIDATOR
        }
    }
}

pub struct Property {
    pub name: String,
    pub description: String,
    pub is_required: bool,
    pub is_sensitive: bool,
    pub supports_expr_lang: bool,
    pub default_value: Option<String>,
    pub validator: StandardPropertyValidator,
}


#[derive(Debug, Eq, PartialEq)]
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

#[derive(Debug)]
pub struct Property {
    pub name: &'static str,
    pub description: &'static str,
    pub is_required: bool,
    pub is_sensitive: bool,
    pub supports_expr_lang: bool,
    pub default_value: Option<&'static str>,
    pub validator: StandardPropertyValidator,
    pub allowed_values: &'static [&'static str],
    pub allowed_type: &'static str,
}

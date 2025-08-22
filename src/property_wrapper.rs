use std::ptr;
use crate::primitives::{create_bool, StringView};
use minifi_native_sys::MinifiProperty;

pub struct Property {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) is_required: bool,
    pub(crate) is_sensitive: bool,
    pub(crate) supports_expr_lang: bool,
    pub(crate) default_value: Option<String>,
}

impl Property {
    pub const fn new(name: String, description: String, is_required: bool, is_sensitive: bool, supports_expr_lang: bool, default_value: Option<String>) -> Self {
        Self {name, description, is_required, is_sensitive, supports_expr_lang, default_value}
    }
}

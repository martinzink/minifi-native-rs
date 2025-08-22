use minifi_native_sys::{MinifiRelationship};
use crate::primitives::create_string_view;

pub struct Relationship {
    pub c_struct: MinifiRelationship,
}

impl Relationship {
    pub const fn new(name: &'static str, description: &'static str) -> Self {
        Self {
            c_struct: MinifiRelationship {
                name: create_string_view(name),
                description: create_string_view(description),
            },
        }
    }
}
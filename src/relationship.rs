use crate::primitives::StaticStrAsMinifiCStr;
use minifi_native_sys::MinifiRelationship;

pub struct Relationship {
    pub name: &'static str,
    pub description: &'static str,
}

impl Relationship {
    pub(crate) fn create_c_vec(relationships: &[Self]) -> Vec<MinifiRelationship> {
        relationships
            .iter()
            .map(|r| MinifiRelationship {
                name: r.name.as_minifi_c_type(),
                description: r.description.as_minifi_c_type(),
            })
            .collect()
    }
}

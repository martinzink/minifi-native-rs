use super::c_ffi_primitives::StaticStrAsMinifiCStr;
use crate::Relationship;
use minifi_native_sys::MinifiRelationship;

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

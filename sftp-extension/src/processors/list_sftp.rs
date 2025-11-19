use strum_macros::{Display, EnumString, IntoStaticStr, VariantNames};

mod processor_definition;
mod properties;
mod relationships;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, VariantNames)]
    #[strum(serialize_all = "title_case")]
enum ListingStrategy {
    TrackingTimestamps,
    TrackingEntities,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, VariantNames)]
#[strum(serialize_all = "title_case")]
pub(crate) enum EntityTrackingInitialListingTarget {
    TrackingTimeWindow,
    AllAvailable,
}
use minifi_native::{Property, StandardPropertyValidator};
use strum::VariantNames;

use super::ListingStrategy;
use super::EntityTrackingInitialListingTarget;

pub(crate) const LISTING_STRATEGY: Property = Property {
    name: "Listing Strategy",
    description: "Specify how to determine new/updated entities. See each strategy descriptions for detail.",
    is_required: true,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: Some("Tracking Timestamps"),
    validator: StandardPropertyValidator::AlwaysValidValidator,
    allowed_values: &ListingStrategy::VARIANTS,
    allowed_type: "",
};

pub(crate) const REMOTE_PATH: Property = Property {
    name: "Remote Path",
    description: "The fully qualified filename on the remote system",
    is_required: false,
    is_sensitive: false,
    supports_expr_lang: true,
    default_value: None,
    validator: StandardPropertyValidator::AlwaysValidValidator,
    allowed_values: &[],
    allowed_type: "",
};

pub(crate) const SEARCH_RECURSIVELY: Property = Property {
    name: "Search Recursively",
    description: "If true, will pull files from arbitrarily nested subdirectories; otherwise, will not traverse subdirectories",
    is_required: true,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: Some("false"),
    validator: StandardPropertyValidator::BoolValidator,
    allowed_values: &[],
    allowed_type: "",
};

pub(crate) const FOLLOW_SYMLINK: Property = Property {
    name: "Follow symlink",
    description: "If true, will pull even symbolic files and also nested symbolic subdirectories; otherwise, will not read symbolic files and will not traverse symbolic link subdirectories",
    is_required: true,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: Some("false"),
    validator: StandardPropertyValidator::BoolValidator,
    allowed_values: &[],
    allowed_type: "",
};

pub(crate) const FILE_FILTER_REGEX: Property = Property {
    name: "File Filter Regex",
    description: "Provides a Java Regular Expression for filtering Filenames; if a filter is supplied, only files whose names match that Regular Expression will be fetched",
    is_required: false,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: None,
    validator: StandardPropertyValidator::AlwaysValidValidator,
    allowed_values: &[],
    allowed_type: "",
};

pub(crate) const PATH_FILTER_REGEX: Property = Property {
    name: "Path Filter Regex",
    description: "When Search Recursively is true, then only subdirectories whose path matches the given Regular Expression will be scanned",
    is_required: false,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: None,
    validator: StandardPropertyValidator::AlwaysValidValidator,
    allowed_values: &[],
    allowed_type: "",
};

pub(crate) const IGNORE_DOTTED_FILES: Property = Property {
    name: "Ignore Dotted Files",
    description: "If true, files whose names begin with a dot (\".\") will be ignored",
    is_required: true,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: Some("true"),
    validator: StandardPropertyValidator::BoolValidator,
    allowed_values: &[],
    allowed_type: "",
};

pub(crate) const ENTITY_TRACKING_TIME_WINDOW: Property = Property {
    name: "Entity Tracking Time Window",
    description: "Specify how long this processor should track already-listed entities. 'Tracking Entities' strategy can pick any entity whose timestamp is inside the specified time window. For example, if set to '30 minutes', any entity having timestamp in recent 30 minutes will be the listing target when this processor runs. A listed entity is considered 'new/updated' and a FlowFile is emitted if one of following condition meets: 1. does not exist in the already-listed entities, 2. has newer timestamp than the cached entity, 3. has different size than the cached entity. If a cached entity's timestamp becomes older than specified time window, that entity will be removed from the cached already-listed entities. Used by 'Tracking Entities' strategy.",
    is_required: false,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: None,
    validator: StandardPropertyValidator::AlwaysValidValidator,
    allowed_values: &[],
    allowed_type: "",
};

pub(crate) const ENTITY_TRACKING_INITIAL_LISTING_TARGET: Property = Property {
    name: "Entity Tracking Initial Listing Target",
    description: "Specify how initial listing should be handled. Used by 'Tracking Entities' strategy.",
    is_required: false,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: Some("All Available"),
    validator: StandardPropertyValidator::AlwaysValidValidator,
    allowed_values: &EntityTrackingInitialListingTarget::VARIANTS,
    allowed_type: "",
};

pub(crate) const MINIMUM_FILE_AGE: Property = Property {
    name: "Minimum File Age",
    description: "The minimum age that a file must be in order to be pulled; any file younger than this amount of time (according to last modification date) will be ignored",
    is_required: false,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: None,
    validator: StandardPropertyValidator::TimePeriodValidator,
    allowed_values: &[],
    allowed_type: "",
};

pub(crate) const MAXIMUM_FILE_AGE: Property = Property {
    name: "Maximum File Age",
    description: "The maximum age that a file must be in order to be pulled; any file older than this amount of time (according to last modification date) will be ignored",
    is_required: false,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: None,
    validator: StandardPropertyValidator::TimePeriodValidator,
    allowed_values: &[],
    allowed_type: "",
};

pub(crate) const MINIMUM_FILE_SIZE: Property = Property {
    name: "Minimum File Size",
    description: "The minimum size that a file must be in order to be pulled",
    is_required: true,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: None,
    validator: StandardPropertyValidator::DataSizeValidator,
    allowed_values: &[],
    allowed_type: "",
};

pub(crate) const MAXIMUM_FILE_SIZE: Property = Property {
    name: "Maximum File Size",
    description: "The maximum size that a file must be in order to be pulled",
    is_required: false,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: None,
    validator: StandardPropertyValidator::DataSizeValidator,
    allowed_values: &[],
    allowed_type: "",
};

use crate::processors::put_s3_object::{ChecksumAlgorithm, ServerSideEncryption, StorageClass};
use minifi_native::{Property, StandardPropertyValidator};
use strum::VariantNames;

pub const OBJECT_KEY: Property = Property {
    name: "Object Key",
    description: "The key of the S3 object. If none is given the filename attribute will be used by default.",
    is_required: false,
    is_sensitive: false,
    supports_expr_lang: true,
    default_value: None,
    validator: StandardPropertyValidator::AlwaysValidValidator,
    allowed_values: &[],
    allowed_type: "",
};

pub const CONTENT_TYPE: Property = Property {
    name: "Content Type",
    description: "Sets the Content-Type HTTP header indicating the type of content stored in the associated object. The value of this header is a standard MIME type. If no content type is provided the default content type \"application/octet-stream\" will be used.",
    is_required: false,
    is_sensitive: false,
    supports_expr_lang: true,
    default_value: Some("application/octet-stream"),
    validator: StandardPropertyValidator::AlwaysValidValidator,
    allowed_values: &[],
    allowed_type: "",
};

pub const STORAGE_CLASS: Property = Property {
    name: "Storage Class",
    description: "AWS S3 Storage Class",
    is_required: true,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: Some(StorageClass::Standard.into_str()),
    validator: StandardPropertyValidator::AlwaysValidValidator,
    allowed_values: &StorageClass::VARIANTS,
    allowed_type: "",
};

pub const SERVER_SIDE_ENCRYPTION: Property = Property {
    name: "Server Side Encryption",
    description: "Specifies the algorithm used for server side encryption.",
    is_required: true,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: Some(ServerSideEncryption::None.into_str()),
    validator: StandardPropertyValidator::AlwaysValidValidator,
    allowed_values: &ServerSideEncryption::VARIANTS,
    allowed_type: "",
};

pub const FULL_CONTROL_USER_LIST: Property = Property {
    name: "FullControl User List",
    description: "A comma-separated list of Amazon User ID's or E-mail addresses that specifies who should have Full Control for an object.",
    is_required: false,
    is_sensitive: false,
    supports_expr_lang: true,
    default_value: None,
    validator: StandardPropertyValidator::NonBlankValidator,
    allowed_values: &[],
    allowed_type: "",
};

pub const READ_PERMISSION_USER_LIST: Property = Property {
    name: "Read Permission User List",
    description: "A comma-separated list of Amazon User ID's or E-mail addresses that specifies who should have Read Access for an object.",
    is_required: false,
    is_sensitive: false,
    supports_expr_lang: true,
    default_value: None,
    validator: StandardPropertyValidator::NonBlankValidator,
    allowed_values: &[],
    allowed_type: "",
};

pub const READ_ACL_USER_LIST: Property = Property {
    name: "Read ACL User List",
    description: "A comma-separated list of Amazon User ID's or E-mail addresses that specifies who should have permissions to read the Access Control List for an object.",
    is_required: false,
    is_sensitive: false,
    supports_expr_lang: true,
    default_value: None,
    validator: StandardPropertyValidator::NonBlankValidator,
    allowed_values: &[],
    allowed_type: "",
};

pub const WRITE_ACL_USER_LIST: Property = Property {
    name: "Write ACL User List",
    description: "A comma-separated list of Amazon User ID's or E-mail addresses that specifies who should have permissions to change the Access Control List for an object.",
    is_required: false,
    is_sensitive: false,
    supports_expr_lang: true,
    default_value: None,
    validator: StandardPropertyValidator::NonBlankValidator,
    allowed_values: &[],
    allowed_type: "",
};

pub const CANNED_ACL: Property = Property {
    name: "Canned ACL",
    description: "Amazon Canned ACL for an object. Allowed values: BucketOwnerFullControl, BucketOwnerRead, AuthenticatedRead, PublicReadWrite, PublicRead, Private, AwsExecRead; will be ignored if any other ACL/permission property is specified.",
    is_required: false,
    is_sensitive: false,
    supports_expr_lang: true,
    default_value: None,
    validator: StandardPropertyValidator::AlwaysValidValidator,
    allowed_values: &[],
    allowed_type: "",
};

pub const USE_PATH_STYLE_ACCESS: Property = Property {
    name: "Use Path Style Access",
    description: "Path-style access can be enforced by setting this property to true. Set it to true if your endpoint does not support virtual-hosted-style requests, only path-style requests.",
    is_required: true,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: Some("false"),
    validator: StandardPropertyValidator::BoolValidator,
    allowed_values: &[],
    allowed_type: "",
};

pub const MULTIPART_THRESHOLD: Property = Property {
    name: "Multipart Threshold",
    description: "Specifies the file size threshold for switch from the PutS3Object API to the PutS3MultipartUpload API. Flow files bigger than this limit will be sent using the multipart process. The valid range is 5MB to 5GB.",
    is_required: true,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: Some("5 GB"),
    validator: StandardPropertyValidator::DataSizeValidator,
    allowed_values: &[],
    allowed_type: "",
};

pub const MULTIPART_PART_SIZE: Property = Property {
    name: "Multipart Part Size",
    description: "Specifies the part size for use when the PutS3Multipart Upload API is used. Flow files will be broken into chunks of this size for the upload process, but the last part sent can be smaller since it is not padded. The valid range is 5MB to 5GB.",
    is_required: true,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: Some("5 GB"),
    validator: StandardPropertyValidator::DataSizeValidator,
    allowed_values: &[],
    allowed_type: "",
};

pub const MULTIPART_UPLOAD_AGE_OFF_INTERVAL: Property = Property {
    name: "Multipart Upload AgeOff Interval",
    description: "Specifies the interval at which existing multipart uploads in AWS S3 will be evaluated for ageoff. When processor is triggered it will initiate the ageoff evaluation if this interval has been exceeded.",
    is_required: true,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: Some("60 min"),
    validator: StandardPropertyValidator::TimePeriodValidator,
    allowed_values: &[],
    allowed_type: "",
};

pub const MULTIPART_UPLOAD_MAX_AGE_THRESHOLD: Property = Property {
    name: "Multipart Upload Max Age Threshold",
    description: "Specifies the maximum age for existing multipart uploads in AWS S3. When the ageoff process occurs, any upload older than this threshold will be aborted.",
    is_required: true,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: Some("7 days"),
    validator: StandardPropertyValidator::TimePeriodValidator,
    allowed_values: &[],
    allowed_type: "",
};

pub const CHECKSUM_ALGORITHM: Property = Property {
    name: "Checksum Algorithm",
    description: "Checksum algorithm used to verify the uploaded object.",
    is_required: true,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: Some("CRC64NVME"),
    validator: StandardPropertyValidator::AlwaysValidValidator,
    allowed_values: &ChecksumAlgorithm::VARIANTS,
    allowed_type: "",
};

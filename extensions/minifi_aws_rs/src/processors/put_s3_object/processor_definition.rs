use super::{PutS3Object, properties, relationships};
use minifi_native::{
    OutputAttribute, ProcessorDefinition, ProcessorInputRequirement, Property, Relationship,
};

impl ProcessorDefinition for PutS3Object {
    const DESCRIPTION: &'static str = "Puts FlowFiles to an Amazon S3 Bucket. The upload uses either the PutS3Object method or the PutS3MultipartUpload method. The PutS3Object method sends the file in a single synchronous call, but it has a 5GB size limit. Larger files are sent using the PutS3MultipartUpload method. This multipart process saves state after each step so that a large upload can be resumed with minimal loss if the processor or cluster is stopped and restarted. A multipart upload consists of three steps: 1) initiate upload, 2) upload the parts, and 3) complete the upload. For multipart uploads, the processor saves state locally tracking the upload ID and parts uploaded, which must both be provided to complete the upload. The AWS libraries select an endpoint URL based on the AWS region, but this can be overridden with the 'Endpoint Override URL' property for use with other S3-compatible endpoints. The S3 API specifies that the maximum file size for a PutS3Object upload is 5GB. It also requires that parts in a multipart upload must be at least 5MB in size, except for the last part. These limits establish the bounds for the Multipart Upload Threshold and Part Size properties.";
    const INPUT_REQUIREMENT: ProcessorInputRequirement = ProcessorInputRequirement::Required;
    const SUPPORTS_DYNAMIC_PROPERTIES: bool = true;
    const SUPPORTS_DYNAMIC_RELATIONSHIPS: bool = false;
    const OUTPUT_ATTRIBUTES: &'static [OutputAttribute] = &[];
    const RELATIONSHIPS: &'static [Relationship] =
        &[relationships::SUCCESS, relationships::FAILURE];
    const PROPERTIES: &'static [Property] = &[
        properties::OBJECT_KEY,
        properties::CONTENT_TYPE,
        properties::STORAGE_CLASS,
        properties::SERVER_SIDE_ENCRYPTION,
        properties::FULL_CONTROL_USER_LIST,
        properties::READ_PERMISSION_USER_LIST,
        properties::READ_ACL_USER_LIST,
        properties::WRITE_ACL_USER_LIST,
        properties::CANNED_ACL,
        properties::USE_PATH_STYLE_ACCESS,
        properties::MULTIPART_THRESHOLD,
        properties::MULTIPART_PART_SIZE,
        properties::MULTIPART_UPLOAD_AGE_OFF_INTERVAL,
        properties::MULTIPART_UPLOAD_MAX_AGE_THRESHOLD,
        properties::CHECKSUM_ALGORITHM,
    ];
}

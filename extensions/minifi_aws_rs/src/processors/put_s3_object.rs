use aws_sdk_s3::primitives::ByteStream;
use minifi_native::macros::{ComponentIdentifier, DefaultMetrics, NoAdvancedProcessorFeatures};
use minifi_native::{
    ComponentIdentifier, FlowFileTransform, GetAttribute, GetControllerService, GetProperty,
    Logger, MinifiError, OutputAttribute, ProcessorDefinition, ProcessorInputRequirement, Property,
    Relationship, Schedule, StandardPropertyValidator, TransformedFlowFile, error, info,
};
use std::collections::HashMap;

use crate::controller_services::aws_credential_service::AwsCredentialServiceRs;

pub(crate) const BUCKET: Property = Property {
    name: "Bucket",
    description: "The Amazon S3 bucket name.",
    is_required: true,
    is_sensitive: false,
    supports_expr_lang: true,
    default_value: None,
    validator: StandardPropertyValidator::AlwaysValidValidator,
    allowed_values: &[],
    allowed_type: "",
};

pub(crate) const OBJECT_KEY: Property = Property {
    name: "Object Key",
    description: "The key of the S3 object. If not set, the 'filename' attribute will be used.",
    is_required: false, // We fallback to filename
    is_sensitive: false,
    supports_expr_lang: true,
    default_value: None,
    validator: StandardPropertyValidator::AlwaysValidValidator,
    allowed_values: &[],
    allowed_type: "",
};

pub(crate) const AWS_CREDENTIALS_PROVIDER_SERVICE: Property = Property {
    name: "AWS Credentials Provider service",
    description: "The Controller Service used to obtain AWS credentials.",
    is_required: true,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: None,
    validator: StandardPropertyValidator::AlwaysValidValidator,
    allowed_values: &[],
    allowed_type: AwsCredentialServiceRs::CLASS_NAME,
};

pub(crate) const ENDPOINT_OVERRIDE_URL: Property = Property {
    name: "Endpoint Override URL",
    description: "Endpoint URL to use instead of the AWS default including scheme, host, port, and path. Often used for LocalStack or MinIO.",
    is_required: false,
    is_sensitive: false,
    supports_expr_lang: true,
    default_value: None,
    validator: StandardPropertyValidator::AlwaysValidValidator,
    allowed_values: &[],
    allowed_type: "",
};

pub(crate) const REGION: Property = Property {
    name: "Region",
    description: "AWS Region",
    is_required: false,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: Some("us-west-2"),
    validator: StandardPropertyValidator::AlwaysValidValidator,
    allowed_values: &[],
    allowed_type: "",
};

pub(crate) const SUCCESS: Relationship = Relationship {
    name: "success",
    description: "FlowFiles are routed to success after being successfully copied to Amazon S3",
};

pub(crate) const FAILURE: Relationship = Relationship {
    name: "failure",
    description: "FlowFiles are routed to failure if the Amazon S3 service cannot be reached or the file cannot be uploaded",
};

#[derive(Debug, ComponentIdentifier, DefaultMetrics, NoAdvancedProcessorFeatures)]
pub(crate) struct PutS3ObjectRs {
    runtime: tokio::runtime::Runtime,
}

impl Schedule for PutS3ObjectRs {
    fn schedule<P: GetProperty, L: Logger>(_context: &P, _logger: &L) -> Result<Self, MinifiError>
    where
        Self: Sized,
    {
        let runtime = tokio::runtime::Runtime::new().map_err(|e| {
            MinifiError::schedule_err(format!("Failed to create tokio runtime: {}", e))
        })?;

        Ok(Self { runtime })
    }
}

impl FlowFileTransform for PutS3ObjectRs {
    fn transform<
        'a,
        Context: GetProperty + GetControllerService + GetAttribute,
        LoggerImpl: Logger,
    >(
        &self,
        context: &Context,
        input_stream: &'a mut dyn minifi_native::InputStream,
        logger: &LoggerImpl,
    ) -> Result<TransformedFlowFile<'a>, MinifiError> {
        let region = context.get_property(&REGION)?.expect("required property");
        // 1. Resolve Target Properties
        let bucket = context
            .get_property(&BUCKET)?
            .ok_or_else(|| MinifiError::trigger_err("Bucket must be set"))?;

        let object_key = context
            .get_property(&OBJECT_KEY)?
            .or_else(|| context.get_attribute("filename").unwrap_or_default())
            .ok_or_else(|| {
                MinifiError::trigger_err("No Object Key provided and filename attribute is missing")
            })?;

        let creds_service = context
            .get_controller_service::<AwsCredentialServiceRs>(&AWS_CREDENTIALS_PROVIDER_SERVICE)?
            .ok_or_else(|| MinifiError::trigger_err("AwsCredentialServiceRs is required"))?;
        let mut s3_client = creds_service.get_client()?;

        if let Some(endpoint) = context.get_property(&ENDPOINT_OVERRIDE_URL)? {
            let config_builder = s3_client
                .config()
                .to_builder()
                .endpoint_url(endpoint)
                .force_path_style(true);
            s3_client = aws_sdk_s3::Client::from_conf(config_builder.build());
        }

        let mut buffer = Vec::new();
        input_stream.read_to_end(&mut buffer).map_err(|e| {
            MinifiError::trigger_err(format!("Failed to read flow file content: {}", e))
        })?;
        info!(logger, "{:?}", buffer);
        let byte_stream = ByteStream::from(buffer);

        let upload_result = self.runtime.block_on(async {
            s3_client
                .put_object()
                .bucket(&bucket)
                .key(&object_key)
                .body(byte_stream)
                .send()
                .await
        });

        match upload_result {
            Ok(output) => {
                let mut attributes = HashMap::new();
                attributes.insert("s3.bucket".to_string(), bucket);
                attributes.insert("s3.key".to_string(), object_key);
                if let Some(etag) = output.e_tag() {
                    attributes.insert("s3.etag".to_string(), etag.to_string());
                }
                if let Some(version) = output.version_id() {
                    attributes.insert("s3.version".to_string(), version.to_string());
                }

                Ok(TransformedFlowFile::new(&SUCCESS, None, attributes))
            }
            Err(e) => {
                error!(logger, "Failed to upload to S3: {:?}", e);
                Ok(TransformedFlowFile::route_without_changes(&FAILURE))
            }
        }
    }
}

impl ProcessorDefinition for PutS3ObjectRs {
    const DESCRIPTION: &'static str = "Puts FlowFiles to an Amazon S3 Bucket";
    const INPUT_REQUIREMENT: ProcessorInputRequirement = ProcessorInputRequirement::Required;
    const SUPPORTS_DYNAMIC_PROPERTIES: bool = false;
    const SUPPORTS_DYNAMIC_RELATIONSHIPS: bool = false;
    const OUTPUT_ATTRIBUTES: &'static [OutputAttribute] = &[
        OutputAttribute {
            name: "s3.bucket",
            relationships: &[SUCCESS.name],
            description: "The S3 bucket where the object was put in S3",
        },
        OutputAttribute {
            name: "s3.key",
            relationships: &[SUCCESS.name],
            description: "The S3 key of the object put into S3",
        },
        OutputAttribute {
            name: "s3.etag",
            relationships: &[SUCCESS.name],
            description: "The ETag that can be used to verify the object",
        },
        OutputAttribute {
            name: "s3.version",
            relationships: &[SUCCESS.name],
            description: "The version of the object put into S3",
        },
    ];
    const RELATIONSHIPS: &'static [Relationship] = &[SUCCESS, FAILURE];
    const PROPERTIES: &'static [Property] = &[
        BUCKET,
        OBJECT_KEY,
        AWS_CREDENTIALS_PROVIDER_SERVICE,
        ENDPOINT_OVERRIDE_URL,
        REGION,
    ];
}

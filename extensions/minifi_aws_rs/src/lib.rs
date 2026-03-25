use crate::controller_services::aws_credential_service::AwsCredentialServiceRs;
use crate::processors::put_s3_object::PutS3ObjectRs;
use minifi_native::{Concurrent, FlowFileTransformProcessorType};

mod controller_services;
mod processors;

minifi_native::declare_minifi_extension!(
    processors: [
        (FlowFileTransformProcessorType, Concurrent, PutS3ObjectRs),
    ],
    controllers: [
        AwsCredentialServiceRs,
    ]
);

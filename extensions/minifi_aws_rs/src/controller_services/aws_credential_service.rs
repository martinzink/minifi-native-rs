use aws_credential_types::Credentials;
use aws_credential_types::provider::SharedCredentialsProvider;
use aws_sdk_s3::Client;
use minifi_native::macros::ComponentIdentifier;
use minifi_native::{
    ControllerServiceDefinition, EnableControllerService, GetProperty, Logger, MinifiError,
    Property, StandardPropertyValidator,
};

pub(crate) const USE_DEFAULT_CREDENTIALS: Property = Property {
    name: "Use Default Credentials",
    description: "If true, uses the Default Credential chain, including EC2 instance profiles or roles, environment variables, default user credentials, etc.",
    is_required: false,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: Some("false"),
    validator: StandardPropertyValidator::BoolValidator,
    allowed_values: &[],
    allowed_type: "",
};

pub(crate) const ACCESS_KEY: Property = Property {
    name: "Access Key",
    description: "Specifies the AWS Access Key.",
    is_required: false,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: None,
    validator: StandardPropertyValidator::AlwaysValidValidator,
    allowed_values: &[],
    allowed_type: "",
};

pub(crate) const SECRET_KEY: Property = Property {
    name: "Secret Key",
    description: "Specifies the AWS Access Key.",
    is_required: false,
    is_sensitive: true,
    supports_expr_lang: false,
    default_value: None,
    validator: StandardPropertyValidator::AlwaysValidValidator,
    allowed_values: &[],
    allowed_type: "",
};

#[derive(Debug, ComponentIdentifier)]
pub(crate) struct AwsCredentialServiceRs {
    provider: SharedCredentialsProvider,
    runtime: tokio::runtime::Runtime,
}

impl AwsCredentialServiceRs {
    pub fn get_client(&self) -> Result<Client, MinifiError> {
        let provider = self.provider.clone();

        let sdk_config = self.runtime.block_on(async {
            aws_config::defaults(aws_config::BehaviorVersion::latest())
                .credentials_provider(provider)
                .region("us-west-2")
                .load()
                .await
        });

        Ok(Client::new(&sdk_config))
    }
}

impl EnableControllerService for AwsCredentialServiceRs {
    fn enable<Ctx: GetProperty, L: Logger>(context: &Ctx, _logger: &L) -> Result<Self, MinifiError>
    where
        Self: Sized,
    {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| {
                MinifiError::controller_service_err(format!(
                    "Failed to create tokio runtime: {}",
                    e
                ))
            })?;

        let access_key = context.get_property(&ACCESS_KEY)?;
        let secret_key = context.get_property(&SECRET_KEY)?;
        let use_default_credentials = context
            .get_bool_property(&USE_DEFAULT_CREDENTIALS)?
            .expect("required property");

        let provider = if use_default_credentials {
            let sdk_config = runtime.block_on(async {
                aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await
            });

            if let Some(creds_provider) = sdk_config.credentials_provider() {
                creds_provider.clone()
            } else {
                return Err(MinifiError::controller_service_err(
                    "Default credentials enabled, but no AWS provider chain found in the environment.",
                ));
            }
        } else if let (Some(ak), Some(sk)) = (&access_key, &secret_key) {
            let creds = Credentials::new(
                ak.clone(),
                sk.clone(),
                None,
                None,
                "minifi-static-credentials",
            );
            SharedCredentialsProvider::new(creds)
        } else {
            return Err(MinifiError::controller_service_err(
                "AWS Credentials not fully configured and default credentials not enabled.",
            ));
        };

        Ok(Self { provider, runtime })
    }
}

impl ControllerServiceDefinition for AwsCredentialServiceRs {
    const DESCRIPTION: &'static str = "Manages the Amazon Web Services (AWS) credentials for an AWS account. This allows for multiple AWS credential services to be defined. This also allows for multiple AWS related processors to reference this single controller service so that AWS credentials can be managed and controlled in a central location.";
    const PROPERTIES: &'static [Property] = &[USE_DEFAULT_CREDENTIALS, ACCESS_KEY, SECRET_KEY];
}

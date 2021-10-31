//! This module contains the models used by this crate.
//!
//! Disclaimer: a few (if not all) of them has been copied from the
//! the official AWS Lambda Runtime project. All rights reserved to its
//! original authors.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Context {
    /// The AWS request ID generated by the Lambda service.
    pub request_id: String,
    /// The execution deadline for the current invocation in milliseconds.
    pub deadline: u64,
    /// The ARN of the Lambda function being invoked.
    pub invoked_function_arn: String,
    /// The X-Ray trace ID for the current invocation.
    pub xray_trace_id: String,
    /// The client context object sent by the AWS mobile SDK. This field is
    /// empty unless the function is invoked using an AWS mobile SDK.
    pub client_context: Option<ClientContext>,
    /// The Cognito identity that invoked the function. This field is empty
    /// unless the invocation request to the Lambda APIs was made using AWS
    /// credentials issues by Amazon Cognito Identity Pools.
    pub identity: Option<CognitoIdentity>,
    /// Lambda function configuration from the local environment variables.
    /// Includes information such as the function name, memory allocation,
    /// version, and log streams.
    pub env_config: Config,
    // Number of times invoked
    pub invocations: u32,
}

/// Client context sent by the AWS Mobile SDK.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClientContext {
    /// Information about the mobile application invoking the function.
    pub client: ClientApplication,
    /// Custom properties attached to the mobile event context.
    #[serde(default)]
    pub custom: HashMap<String, String>,
    /// Environment settings from the mobile client.
    #[serde(default, alias = "env")]
    pub environment: HashMap<String, String>,
}

/// AWS Mobile SDK client fields.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClientApplication {
    /// The mobile app installation id
    #[serde(alias = "installationId")]
    pub installation_id: String,
    /// The app title for the mobile app as registered with AWS' mobile services.
    #[serde(alias = "appTitle")]
    pub app_title: String,
    /// The version name of the application as registered with AWS' mobile services.
    #[serde(alias = "appVersionName")]
    pub app_version_name: String,
    /// The app version code.
    #[serde(alias = "appVersionCode")]
    pub app_version_code: String,
    /// The package name for the mobile application invoking the function
    #[serde(alias = "appPackageName")]
    pub app_package_name: String,
}

/// Cognito identity information sent with the event
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CognitoIdentity {
    /// The unique identity id for the Cognito credentials invoking the function.
    #[serde(alias = "identityId")]
    pub identity_id: String,
    /// The identity pool id the caller is "registered" with.
    #[serde(alias = "identityPoolId")]
    pub identity_pool_id: String,
}

/// Configuration derived from environment variables.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    /// The host and port of the [runtime API](https://docs.aws.amazon.com/lambda/latest/dg/runtimes-api.html).
    pub endpoint: String,
    /// The name of the function.
    pub function_name: String,
    /// The amount of memory available to the function in MB.
    pub memory: i32,
    /// The version of the function being executed.
    pub version: String,
    /// The name of the Amazon CloudWatch Logs stream for the function.
    pub log_stream: String,
    /// The name of the Amazon CloudWatch Logs group for the function.
    pub log_group: String,
}

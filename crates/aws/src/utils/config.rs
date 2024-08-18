use aws_config::{
  Region,
  {meta::region::RegionProviderChain, BehaviorVersion, SdkConfig},
};
use std::env;

use crate::constants::{defaults::DEFAULT_REGION, env::AWS_ACCOUNT_ID};

/// Gets the AWS region provider.
///
/// # Returns
/// The AWS region provider.
///
/// # Example
/// ```
/// let region_provider = get_region_provider();
///
/// ```
///
pub fn get_region_provider() -> RegionProviderChain {
  return RegionProviderChain::default_provider().or_else(DEFAULT_REGION);
}

/// Gets the AWS region.
///
/// # Returns
/// The AWS region.
///
/// # Example
/// ```
/// let region = get_region().await;
///
/// ```
///
pub async fn get_region() -> Region {
  return get_region_provider().region().await.expect("No AWS region provided to the application.");
}

/// Gets the AWS Account Id.
///
/// # Returns
/// The AWS Account Id.
///
/// # Example
/// ```
/// let account_id = get_aws_account_id();
/// assert_eq!(account_id, "123456789012");
/// ```
///
pub fn get_aws_account_id() -> String {
  return env::var(AWS_ACCOUNT_ID)
    .expect("No AWS Account Id could be found in the `AWS_ACCOUNT_ID` enviroment variable.");
}

/// Gets the AWS config.
///
/// # Returns
/// The AWS config.
///
/// # Example
/// ```
/// let config = get_aws_config().await;
/// ```
///
pub async fn get_aws_config() -> SdkConfig {
  return aws_config::defaults(BehaviorVersion::v2024_03_28())
    .region(get_region_provider())
    .load()
    .await;
}

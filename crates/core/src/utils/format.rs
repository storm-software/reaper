use aws_config::meta::region::RegionProviderChain;
use std::env;

use crate::constants::aws::DEFAULT_REGION;
use crate::constants::env::AWS_ACCOUNT_ID;

/// Formats a SNS ARN.
///
/// # Arguments
/// * `topic` - The name of the SNS topic.
///
/// # Returns
/// The formatted SNS ARN.
///
/// # Example
/// ```
/// let topic = "my-topic";
///
/// let arn = format_sns_arn(topic);
/// assert_eq!(arn, "arn:aws:sns:us-east-1:123456789012:my-topic");
/// ```
///
pub async fn format_sns_arn(topic: &str) -> String {
  let region = RegionProviderChain::default_provider()
    .or_else(DEFAULT_REGION)
    .region()
    .await
    .expect("No AWS region provided to the application.");
  let account = env::var(AWS_ACCOUNT_ID)
    .expect("No AWS Account Id could be found in the `AWS_ACCOUNT_ID` enviroment variable.");

  return format!("arn:aws:sns:{}:{}:{}", region, account, topic);
}

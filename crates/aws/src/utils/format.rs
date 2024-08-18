use crate::utils::config::{get_aws_account_id, get_region};

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
  return format!("arn:aws:sns:{}:{}:{}", get_region().await, get_aws_account_id(), topic);
}

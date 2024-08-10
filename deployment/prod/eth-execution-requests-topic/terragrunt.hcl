
# ------------------------------------------------------------------------------------------------------
# DEPLOY SNS MODULE
# ------------------------------------------------------------------------------------------------------

terraform {
  source = "git::git@github.com:gruntwork-io/terraform-aws-messaging.git//modules/sns?ref=v0.12.5"
}

inputs = {

  # ----------------------------------------------------------------------------------------------------
  # REQUIRED VARIABLES
  # ----------------------------------------------------------------------------------------------------

  # The name of the SNS topic.
  name = "trading.eth.execution.requests"

  # ----------------------------------------------------------------------------------------------------
  # OPTIONAL VARIABLES
  # ----------------------------------------------------------------------------------------------------

  # A list of IAM ARNs that will be given the rights to publish to the SNS
  # topic.
  allow_publish_accounts = []

  # A list of AWS services that will be given the rights to publish to the SNS
  # topic.
  allow_publish_services = []

  # A list of IAM ARNs that will be given the rights to subscribe to the SNS
  # topic.
  allow_subscribe_accounts = []

  # A list of protocols that are allowed for subscription.
  allow_subscribe_protocols = ["http","https","email","email-json","sms","sqs","application","lambda"]

  # **Requires `enable_fifo = true`.** Flag to enable content-based
  # deduplication for the SNS topic. If set to true, messages with identical
  # content will be treated as duplicates and only delivered once. For more see
  # the [Amazon
  # Docs](https://docs.aws.amazon.com/sns/latest/dg/fifo-message-dedup.html)
  content_based_deduplication = null

  # Enable or disable creation of the resources of this module.
  create_resources = true

  # Delivery policy for sns topic.
  delivery_policy = null

  # The display name of the SNS topic. NOTE: Maximum length is 100 characters.
  display_name = ""

  # Flag to indicate if the SNS topic is FIFO. This will append `.fifo` to the
  # name of the topic.
  enable_fifo = false

  # ARN of the http failure feedback role - when using delivery policy for sns
  # topic.
  http_failure_feedback_role_arn = null

  # ARN of the http success feedback role - when using delivery policy for sns
  # topic.
  http_success_feedback_role_arn = null

  # The ID of an AWS-managed customer master key (CMK) for Amazon SNS or a
  # custom CMK
  kms_master_key_id = null

  # **Requires `enable_fifo = true`.** The number of days (up to 365) for Amazon
  # SNS to retain messages. This will be used to create the archive policy for
  # the SNS topic. For more see the [Amazon
  # Docs](https://docs.aws.amazon.com/sns/latest/dg/message-archiving-and-replay-topic-owner.html)
  message_retention_period = null

  # A map of key value pairs to apply as tags to the SNS topic.
  tags = {
    IAC = "true"
    Environment = "dev"
  }

}


include "root" {
  path = find_in_parent_folders()
}

# Indicate where to source the terraform module from.
# The URL used here is a shorthand for
# "tfr://registry.terraform.io/terraform-aws-modules/vpc/aws?version=5.8.1".
# Note the extra `/` after the protocol is required for the shorthand
# notation.
terraform {
  source = "git::https://github.com/storm-software/storm-ops.git//modules/aws/lambda-rs?ref=main"
}

# # Indicate what region to deploy the resources into
# generate "provider" {
#   path = "provider.tf"
#   if_exists = "overwrite_terragrunt"
#   contents = <<EOF
# provider "aws" {
#   region = "us-east-1"
# }
# EOF
# }

# Indicate the input values to use for the variables of the module.
inputs = {
  name = "eth-execution-lambda"

  aws_region = "us-east-1"
  log_level = "debug"
  dist_path = "dist/target/eth-execution-telegram-bot"

  tags = {
    IAC = "true"
    Environment = "dev"
  }
}


# resource "aws_sns_topic" "eth_execution_requests" {
#   name                        = "eth_execution_requests"
#   fifo_topic                  = true
#   content_based_deduplication = true
# }

include "root" {
  path = find_in_parent_folders()
}

# Indicate where to source the terraform module from.
# The URL used here is a shorthand for
# "tfr://registry.terraform.io/terraform-aws-modules/vpc/aws?version=5.8.1".
# Note the extra `/` after the protocol is required for the shorthand
# notation.
terraform {
  source = "git::https://github.com/storm-software/storm-ops.git//terraform-modules/aws/lambda-rs?ref=main"
}

Indicate the input values to use for the variables of the module.
inputs = {
  name = "eth-execution-telegram-bot"

  aws_region = "us-east-1"
  log_level = "debug"
  dist_path = "/home/runner/work/storm-trading/storm-trading/dist/target/lambda/eth-execution-telegram-bot/bootstrap.zip"
  project_path = "/home/runner/work/storm-trading/storm-trading/apps/eth-execution-telegram-bot"

  tags = {
    IAC = "true"
    Environment = "prod"
    Team = "trading"
  }
}


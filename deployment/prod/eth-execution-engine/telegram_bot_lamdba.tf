locals {
  name = "eth-execution-telegram-bot"
  aws_region = "us-east-1"
  log_level = "debug"
  dist_path = "/home/runner/work/storm-trading/storm-trading/dist/target/eth-execution-telegram-bot/bootstrap.zip"
  project_path = "/home/runner/work/storm-trading/storm-trading/apps/eth-execution-telegram-bot"
}

resource "aws_iam_role" "lambda_role" {
name   = "${local.name}_Lambda_Function_Role"
assume_role_policy = <<EOF
{
 "Version": "2012-10-17",
 "Statement": [
   {
     "Action": "sts:AssumeRole",
     "Principal": {
       "Service": "lambda.amazonaws.com"
     },
     "Effect": "Allow",
     "Sid": ""
   }
 ]
}
EOF
}

resource "aws_iam_policy" "iam_policy_for_lambda" {
 name         = "${local.name}_aws_iam_policy"
 path         = "/"
 description  = "AWS IAM Policy for managing aws lambda role"
 policy = <<EOF
{
 "Version": "2012-10-17",
 "Statement": [
   {
     "Action": [
       "logs:CreateLogGroup",
       "logs:CreateLogStream",
       "logs:PutLogEvents"
     ],
     "Resource": "arn:aws:logs:*:*:*",
     "Effect": "Allow"
   },
   {
     "Action": [
       "secretsmanager:GetSecretValue"
     ],
     "Resource": "arn:aws:secretsmanager:*:*:*",
     "Effect": "Allow"
   },
   {
     "Action": [
       "sns:Publish"
     ],
     "Resource": "arn:aws:sns:*:*:*",
     "Effect": "Allow"
   }
 ]
}
EOF
}

resource "aws_iam_role_policy_attachment" "attach_iam_policy_to_iam_role" {
 role        = aws_iam_role.lambda_role.name
 policy_arn  = aws_iam_policy.iam_policy_for_lambda.arn
}

resource "random_uuid" "lambda_src_hash" {
  keepers = {
    for filename in setunion(
      fileset(local.project_path, "**/*.rs"),
      fileset(local.project_path, "Cargo.toml"),
    ):
    filename => filemd5("${local.project_path}/${filename}")
  }
}

# Here is the definition of our lambda function
resource "aws_lambda_function" "lambda_dist" {
  function_name = local.name
#   source_code_hash = data.archive_file.lambda_dist_archive.output_base64sha256
  source_code_hash = "${random_uuid.lambda_src_hash.result}"
  filename = local.dist_path
  handler = "bootstrap"
  package_type = "Zip"
  runtime = "provided.al2023"

  # here we enable debug logging for our Rust run-time environment. We would change
  # this to something less verbose for production.
 environment {
   variables = {
     "RUST_LOG" = local.log_level
   }
 }

 #This attaches the role defined above to this lambda function
 role = aws_iam_role.lambda_role.arn
 depends_on  = [aws_iam_role_policy_attachment.attach_iam_policy_to_iam_role]
}


// The Lambda Function URL that allows direct access to our function
resource "aws_lambda_function_url" "lambda_dist_function" {
   function_name = aws_lambda_function.lambda_dist.function_name
   authorization_type = "NONE"
}

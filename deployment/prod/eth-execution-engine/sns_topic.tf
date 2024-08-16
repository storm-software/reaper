resource "aws_sns_topic" "eth_execution_engine_requests_topic" {
  name = "eth-execution-engine-requests-topic"
  content_based_deduplication = true
}

resource "aws_sqs_queue" "eth_execution_engine_requests_queue" {
    name = "eth-execution-engine-requests-queue"
    redrive_policy  = "{\"deadLetterTargetArn\":\"${aws_sqs_queue.eth_execution_engine_requests_dl_queue.arn}\",\"maxReceiveCount\":5}"
    visibility_timeout_seconds = 300

    tags = {
        Environment = "prod"
    }
}

resource "aws_sqs_queue" "eth_execution_engine_requests_dl_queue" {
    name = "eth-execution-engine-requests-dl-queue"
}

resource "aws_sns_topic_subscription" "eth_execution_engine_requests_sqs_target" {
    topic_arn = "${aws_sns_topic.eth_execution_engine_requests.arn}"
    protocol  = "sqs"
    endpoint  = "${aws_sqs_queue.eth_execution_engine_requests_queue.arn}"
}

resource "aws_sqs_queue_policy" "eth_execution_engine_requests_queue_policy" {
    queue_url = "${aws_sqs_queue.eth_execution_engine_requests_queue.id}"

    policy = <<POLICY
{
  "Version": "2012-10-17",
  "Id": "sqspolicy",
  "Statement": [
    {
      "Sid": "First",
      "Effect": "Allow",
      "Principal": "*",
      "Action": "sqs:SendMessage",
      "Resource": "${aws_sqs_queue.eth_execution_engine_requests_queue.arn}",
      "Condition": {
        "ArnEquals": {
          "aws:SourceArn": "${aws_sns_topic.eth_execution_engine_requests.arn}"
        }
      }
    }
  ]
}
POLICY
}

resource "aws_api_gateway_rest_api" "eth_execution_telegram_bot_api" {
  name        = "eth-execution-telegram-bot-api"
  description = "API Gateway for the Ethereum Execution Engine's Telegram Bot"
}

resource "aws_api_gateway_resource" "eth_execution_telegram_bot_api_proxy_resource" {
  rest_api_id = "${aws_api_gateway_rest_api.eth_execution_telegram_bot_api.id}"
  parent_id   = "${aws_api_gateway_rest_api.eth_execution_telegram_bot_api.root_resource_id}"
  path_part   = "{proxy+}"
}

resource "aws_api_gateway_method" "eth_execution_telegram_bot_api_proxy_method" {
  rest_api_id   = "${aws_api_gateway_rest_api.eth_execution_telegram_bot_api.id}"
  resource_id   = "${aws_api_gateway_resource.eth_execution_telegram_bot_api_proxy_resource.id}"
  http_method   = "ANY"
  authorization = "NONE"
}

resource "aws_api_gateway_integration" "eth_execution_telegram_bot_integration" {
  rest_api_id = "${aws_api_gateway_rest_api.eth_execution_telegram_bot_api.id}"
  resource_id = "${aws_api_gateway_method.eth_execution_telegram_bot_api_proxy_method.resource_id}"
  http_method = "${aws_api_gateway_method.eth_execution_telegram_bot_api_proxy_method.http_method}"

  integration_http_method = "POST"
  type                    = "AWS_PROXY"
  uri                     = "${aws_lambda_function.lambda_dist.invoke_arn}"
}

resource "aws_api_gateway_method" "eth_execution_telegram_bot_proxy_root" {
  rest_api_id   = "${aws_api_gateway_rest_api.eth_execution_telegram_bot_api.id}"
  resource_id   = "${aws_api_gateway_rest_api.eth_execution_telegram_bot_api.root_resource_id}"
  http_method   = "ANY"
  authorization = "NONE"
}

resource "aws_api_gateway_integration" "eth_execution_telegram_bot_integration_root" {
  rest_api_id = "${aws_api_gateway_rest_api.eth_execution_telegram_bot_api.id}"
  resource_id = "${aws_api_gateway_method.eth_execution_telegram_bot_proxy_root.resource_id}"
  http_method = "${aws_api_gateway_method.eth_execution_telegram_bot_proxy_root.http_method}"

  integration_http_method = "POST"
  type                    = "AWS_PROXY"
  uri                     = "${aws_lambda_function.lambda_dist.invoke_arn}"
}

resource "aws_api_gateway_deployment" "eth_execution_telegram_bot_api" {
  depends_on = [
    "aws_api_gateway_integration.eth_execution_telegram_bot_integration",
    "aws_api_gateway_integration.eth_execution_telegram_bot_integration_root",
  ]

  rest_api_id = "${aws_api_gateway_rest_api.eth_execution_telegram_bot_api.id}"
  stage_name  = "execution-engine"
}

resource "aws_lambda_permission" "apigw" {
  statement_id  = "AllowExecutionFromAPIGateway"
  action        = "lambda:InvokeFunction"
  function_name = "${aws_lambda_function.lambda_dist.function_name}"
  principal     = "apigateway.amazonaws.com"

  # The /*/* portion grants access from any method on any resource
  # within the API Gateway "REST API".
  source_arn = "${aws_api_gateway_rest_api.eth_execution_telegram_bot_api.execution_arn}/*/*"
}

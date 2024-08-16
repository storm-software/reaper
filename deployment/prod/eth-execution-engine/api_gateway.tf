resource "aws_api_gateway_rest_api" "eth_execution_engine_api" {
  name        = "eth-execution-engine-api"
  description = "API Gateway for eth-execution-engine"
}

resource "aws_api_gateway_resource" "proxy" {
  rest_api_id = "${aws_api_gateway_rest_api.eth_execution_engine_api.id}"
  parent_id   = "${aws_api_gateway_rest_api.eth_execution_engine_api.root_resource_id}"
  path_part   = "{proxy+}"
}

resource "aws_api_gateway_method" "proxy" {
  rest_api_id   = "${aws_api_gateway_rest_api.eth_execution_engine_api.id}"
  resource_id   = "${aws_api_gateway_resource.proxy.id}"
  http_method   = "ANY"
  authorization = "NONE"
}

resource "aws_api_gateway_integration" "eth_execution_telegram_bot_lambda" {
  rest_api_id = "${aws_api_gateway_rest_api.eth_execution_engine_api.id}"
  resource_id = "${aws_api_gateway_method.proxy.resource_id}"
  http_method = "${aws_api_gateway_method.proxy.http_method}"

  integration_http_method = "POST"
  type                    = "AWS_PROXY"
  uri                     = "${aws_lambda_function.eth_execution_telegram_bot_dist.invoke_arn}"
}

resource "aws_api_gateway_method" "eth_execution_telegram_bot_proxy_root" {
  rest_api_id   = "${aws_api_gateway_rest_api.eth_execution_engine_api.id}"
  resource_id   = "${aws_api_gateway_rest_api.eth_execution_engine_api.root_resource_id}"
  http_method   = "ANY"
  authorization = "NONE"
}

resource "aws_api_gateway_integration" "eth_execution_telegram_bot_lambda_root" {
  rest_api_id = "${aws_api_gateway_rest_api.eth_execution_engine_api.id}"
  resource_id = "${aws_api_gateway_method.eth_execution_telegram_bot_proxy_root.resource_id}"
  http_method = "${aws_api_gateway_method.eth_execution_telegram_bot_proxy_root.http_method}"

  integration_http_method = "POST"
  type                    = "AWS_PROXY"
  uri                     = "${aws_lambda_function.eth_execution_telegram_bot_dist.invoke_arn}"
}

resource "aws_api_gateway_deployment" "eth_execution_engine_api" {
  depends_on = [
    "aws_api_gateway_integration.eth_execution_telegram_bot_lambda",
    "aws_api_gateway_integration.eth_execution_telegram_bot_lambda_root",
  ]

  rest_api_id = "${aws_api_gateway_rest_api.eth_execution_engine_api.id}"
  stage_name  = "execution-engine"
}

resource "aws_lambda_permission" "apigw" {
  statement_id  = "AllowExecutionFromAPIGateway"
  action        = "lambda:InvokeFunction"
  function_name = "${aws_lambda_function.eth_execution_telegram_bot_dist.function_name}"
  principal     = "apigateway.amazonaws.com"

  # The /*/* portion grants access from any method on any resource
  # within the API Gateway "REST API".
  source_arn = "${aws_api_gateway_rest_api.eth_execution_engine_api.execution_arn}/*/*"
}

# GENERIC RESOURCES
resource "aws_apigatewayv2_api" "api" {
  name          = "Payments API"
  description   = "Payments API"
  protocol_type = "HTTP"

  cors_configuration {
    allow_origins = ["*"]
    allow_methods = ["*"]
    allow_headers = ["*"]
  }
}

resource "aws_apigatewayv2_stage" "api_stage" {
  api_id      = aws_apigatewayv2_api.api.id
  name        = "$default"
  auto_deploy = true
}

resource "aws_apigatewayv2_deployment" "api_deployment" {
  api_id      = aws_apigatewayv2_api.api.id
  description = "Payments API deployment"

  lifecycle {
    create_before_destroy = true
  }

  depends_on = [
    aws_apigatewayv2_route.initiate_payment_route,
    aws_apigatewayv2_route.finish_payment_route,
  ]
}

# INITIATE PAYMENT
resource "aws_apigatewayv2_integration" "initiate_payment_integration" {
  api_id           = aws_apigatewayv2_api.api.id
  integration_type = "AWS_PROXY"

  connection_type    = "INTERNET"
  description        = "Initiate Payment"
  integration_method = "POST"
  integration_uri    = aws_lambda_function.initiate_payment_lambda.invoke_arn

  payload_format_version = "2.0"
}

resource "aws_apigatewayv2_route" "initiate_payment_route" {
  api_id    = aws_apigatewayv2_api.api.id
  route_key = "POST /payment/inititate"
  target    = "integrations/${aws_apigatewayv2_integration.initiate_payment_integration.id}"
}

resource "aws_lambda_permission" "initiate_payment_api_permission" {
  function_name = aws_lambda_function.initiate_payment_lambda.function_name
  action        = "lambda:InvokeFunction"
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.api.execution_arn}/*/*"
}

# FINISH PAYMENT
resource "aws_apigatewayv2_integration" "finish_payment_integration" {
  api_id           = aws_apigatewayv2_api.api.id
  integration_type = "AWS_PROXY"

  connection_type    = "INTERNET"
  description        = "Finish Payment"
  integration_method = "POST"
  integration_uri    = aws_lambda_function.finish_payment_lambda.invoke_arn

  payload_format_version = "2.0"
}

resource "aws_apigatewayv2_route" "finish_payment_route" {
  api_id    = aws_apigatewayv2_api.api.id
  route_key = "POST /payment/finish"
  target    = "integrations/${aws_apigatewayv2_integration.finish_payment_integration.id}"
}

resource "aws_lambda_permission" "finish_payment_api_permission" {
  function_name = aws_lambda_function.finish_payment_lambda.function_name
  action        = "lambda:InvokeFunction"
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.api.execution_arn}/*/*"
}

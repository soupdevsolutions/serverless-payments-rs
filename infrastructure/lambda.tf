# INITIATE PAYMENT
resource "aws_lambda_function" "initiate_payment_lambda" {
  function_name = "InitiatePayment"

  source_code_hash = filebase64sha256("data/lambdas/initiate_payment.zip")
  filename         = "data/lambdas/initiate_payment.zip"

  handler = "handler"
  runtime = "provided.al2"

  role = aws_iam_role.initiate_payment.arn

  environment {
    variables = {
      STRIPE_SECRET_KEY   = var.STRIPE_API_KEY
      PAYMENTS_TABLE_NAME = aws_dynamodb_table.payments.name
      DOMAIN              = "http://example.com"
    }
  }
}

# FINISH PAYMENT
resource "aws_lambda_function" "finish_payment_lambda" {
  function_name = "FinishPayment"

  source_code_hash = filebase64sha256("data/lambdas/finish_payment.zip")
  filename         = "data/lambdas/finish_payment.zip"

  handler = "handler"
  runtime = "provided.al2"

  role = aws_iam_role.finish_payment.arn

  environment {
    variables = {
      PAYMENTS_TABLE_NAME   = aws_dynamodb_table.payments.name
      STRIPE_WEBHOOK_SECRET = stripe_webhook_endpoint.successful_payments.secret
    }
  }

  depends_on = [stripe_webhook_endpoint.successful_payments]
}

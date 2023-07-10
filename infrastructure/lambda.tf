# INITIATE PAYMENT
resource "aws_lambda_function" "initiate_payment_lambda" {
  function_name = "InitiatePayment"

  source_code_hash = data.archive_file.initiate_payment_archive.output_base64sha256
  filename         = data.archive_file.initiate_payment_archive.output_path

  handler = "func"
  runtime = "provided"

  role = aws_iam_role.initiate_payment.arn

  environment {
    variables = {
      STRIPE_SECRET_KEY   = var.stripe_api_token
      PAYMENTS_TABLE_NAME = aws_dynamodb_table.payments.name
      DOMAIN              = aws_s3_bucket_website_configuration.payments-client.website_endpoint
    }
  }
}

# FINISH PAYMENT
resource "aws_lambda_function" "finish_payment_lambda" {
  function_name = "FinishPayment"

  source_code_hash = data.archive_file.finish_payment_archive.output_base64sha256
  filename         = data.archive_file.finish_payment_archive.output_path

  handler = "func"
  runtime = "provided"

  role = aws_iam_role.finish_payment.arn

  environment {
    variables = {
      PAYMENTS_TABLE_NAME   = aws_dynamodb_table.payments.name
      STRIPE_SECRET_KEY     = var.stripe_api_token
      STRIPE_WEBHOOK_SECRET = stripe_webhook_endpoint.successful_payments.secret
    }
  }
}
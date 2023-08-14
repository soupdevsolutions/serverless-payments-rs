resource "stripe_webhook_endpoint" "successful_payments" {
  url = format("%s%s", aws_apigatewayv2_stage.invoke_url, "payment/finish")

  enabled_events = [
    "charge.succeeded",
  ]
}
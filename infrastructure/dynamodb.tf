resource "aws_dynamodb_table" "payments" {
  name         = "Payments"
  hash_key     = "id"
  billing_mode = "PAY_PER_REQUEST"

  attribute {
    name = "id"
    type = "S"
  }
}

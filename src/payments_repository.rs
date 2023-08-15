use aws_sdk_dynamodb::{types::AttributeValue, Client};

use crate::domain::Payment;

pub struct PaymentsRepository {
    client: Client,
    table_name: String,
}

impl PaymentsRepository {
    pub fn new(client: Client, table_name: impl Into<String>) -> Self {
        Self {
            client,
            table_name: table_name.into(),
        }
    }

    pub async fn insert_payment(self, payment: Payment) -> Result<(), String> {
        let id = AttributeValue::S(payment.id);
        let amount = AttributeValue::N(payment.amount.to_string());
        let sender = AttributeValue::S(payment.sender);
        let status = AttributeValue::N((payment.status as i8).to_string());

        let _request = self
            .client
            .put_item()
            .table_name(self.table_name)
            .item("id", id)
            .item("amount", amount)
            .item("sender", sender)
            .item("status", status)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}

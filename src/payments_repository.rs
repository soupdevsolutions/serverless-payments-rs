use aws_sdk_dynamodb::{types::AttributeValue, Client};

use crate::payment::{Payment, PaymentStatus};

pub const PAYMENTS_TABLE: &str = "PAYMENTS_TABLE_NAME";

pub struct PaymentsRepository {
    client: Client,
    table_name: String,
}

impl PaymentsRepository {
    pub fn new(client: Client) -> Self {
        let table_name = std::env::var(PAYMENTS_TABLE)
            .unwrap_or_else(|_| panic!("{} variable not set", PAYMENTS_TABLE));

        Self { client, table_name }
    }

    pub async fn insert_payment(&self, payment: Payment) -> Result<(), String> {
        let id = AttributeValue::S(payment.id);
        let amount = AttributeValue::N(payment.amount.to_string());
        let sender = AttributeValue::S(payment.sender);
        let status = AttributeValue::N((payment.status as i8).to_string());

        self.client
            .put_item()
            .table_name(&self.table_name)
            .item("id", id)
            .item("amount", amount)
            .item("sender", sender)
            .item("status", status)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn update_payment_status(
        &self,
        payment_id: &str,
        new_status: PaymentStatus,
    ) -> Result<(), String> {
        let id = AttributeValue::S(String::from(payment_id));
        let status = AttributeValue::N((new_status as i8).to_string());

        let _request = self
            .client
            .update_item()
            .table_name(&self.table_name)
            .key("id", id)
            .update_expression("SET #status = :status")
            .expression_attribute_names("#status", "status")
            .expression_attribute_values(":status", status)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}

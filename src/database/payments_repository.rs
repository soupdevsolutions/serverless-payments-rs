use aws_sdk_dynamodb::{types::AttributeValue, Client};
use tokio::sync::OnceCell;

use crate::{
    domain::{Payment, PaymentStatus},
    environment::{get_env_var, PAYMENTS_TABLE},
};

static PAYMENTS_REPOSITORY: OnceCell<PaymentsRepository> = OnceCell::const_new();

#[derive(Clone)]
pub struct PaymentsRepository {
    client: Client,
    table_name: String,
}

impl PaymentsRepository {
    pub async fn get() -> PaymentsRepository {
        PAYMENTS_REPOSITORY
            .get_or_init(|| async {
                let client = Client::new(&aws_config::load_from_env().await);
                PaymentsRepository::new(client)
            })
            .await
            .clone()
    }

    fn new(client: Client) -> Self {
        let table_name = get_env_var(PAYMENTS_TABLE)
            .unwrap_or_else(|_| format!("{} variable not set", PAYMENTS_TABLE));

        Self { client, table_name }
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

    pub async fn update_payment_status(
        self,
        payment_id: &str,
        new_status: PaymentStatus,
    ) -> Result<(), String> {
        let id = AttributeValue::S(String::from(payment_id));
        let status = AttributeValue::N((new_status as i8).to_string());

        let _request = self
            .client
            .update_item()
            .table_name(self.table_name)
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

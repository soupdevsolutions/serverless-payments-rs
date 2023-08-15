use crate::domain::{Payment, PaymentRequest, PaymentStatus};
use stripe::{Client, CreateCustomer, CreatePaymentIntent, Currency, Customer, PaymentIntent};

pub struct PaymentClient {
    stripe_client: stripe::Client,
}

impl PaymentClient {
    pub fn new(secret_key: impl Into<String>) -> Self {
        Self {
            stripe_client: Client::new(secret_key),
        }
    }

    pub async fn initiate_payment(
        self,
        payment_request: PaymentRequest,
    ) -> Result<Payment, String> {
        let customer = Customer::create(
            &self.stripe_client,
            CreateCustomer {
                name: Some(&payment_request.sender),
                description: Some(&format!("Payment from {}", payment_request.sender)),
                ..Default::default()
            },
        )
        .await
        .map_err(|e| e.to_string())?;

        let mut create_intent = CreatePaymentIntent::new(payment_request.amount, Currency::USD);
        create_intent.customer = Some(customer.id);

        PaymentIntent::create(&self.stripe_client, create_intent)
            .await
            .map_err(|e| e.to_string())?;

        Ok(payment_request.into())
    }
}

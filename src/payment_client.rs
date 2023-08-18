use crate::{
    domain::PaymentRequest,
    environment::{get_env_var, DOMAIN},
};
use stripe::{
    CheckoutSession, Client, CreateCheckoutSession, CreateCheckoutSessionLineItems,
    CreateCheckoutSessionLineItemsPriceData, CreateCheckoutSessionLineItemsPriceDataProductData,
    Currency,
};

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
        payment_request: &PaymentRequest,
    ) -> Result<Option<String>, String> {
        let domain = get_env_var(DOMAIN)?;

        let mut create_session_params = CreateCheckoutSession::new(&domain);
        create_session_params.line_items = Some(vec![CreateCheckoutSessionLineItems {
            price_data: Some(CreateCheckoutSessionLineItemsPriceData {
                currency: Currency::USD,
                product_data: Some(CreateCheckoutSessionLineItemsPriceDataProductData {
                    name: format!("Payment from {}", payment_request.sender),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        }]);
        let session = CheckoutSession::create(&self.stripe_client, create_session_params)
            .await
            .map_err(|e| e.to_string())?;

        Ok(session.url)
    }
}

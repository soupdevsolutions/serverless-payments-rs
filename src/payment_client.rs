use crate::{
    environment::{DOMAIN, STRIPE_SECRET_KEY},
    payment::Payment,
};
use stripe::{
    CheckoutSession, CheckoutSessionMode, Client, CreateCheckoutSession,
    CreateCheckoutSessionLineItems, CreateCheckoutSessionLineItemsPriceData,
    CreateCheckoutSessionLineItemsPriceDataProductData, CreateCheckoutSessionPaymentIntentData,
    Currency, Metadata,
};

pub struct PaymentClient {
    stripe_client: stripe::Client,
}

impl Default for PaymentClient {
    fn default() -> Self {
        Self::new()
    }
}

impl PaymentClient {
    pub fn new() -> Self {
        let secret_key = std::env::var(STRIPE_SECRET_KEY)
            .unwrap_or_else(|_| format!("{} variable not set", STRIPE_SECRET_KEY));
        Self {
            stripe_client: Client::new(secret_key),
        }
    }

    #[tracing::instrument(skip(self, payment), fields(sender = %payment.sender, amount = %payment.amount))]
    pub async fn initiate_payment(&self, payment: &Payment) -> Result<String, String> {
        let domain = std::env::var(DOMAIN).map_err(|e| e.to_string())?;

        let mut create_session_params = CreateCheckoutSession::new(&domain);
        create_session_params.line_items = Some(vec![CreateCheckoutSessionLineItems {
            price_data: Some(CreateCheckoutSessionLineItemsPriceData {
                currency: Currency::USD,
                product_data: Some(CreateCheckoutSessionLineItemsPriceDataProductData {
                    name: format!("Payment from {}", payment.sender),
                    ..Default::default()
                }),
                unit_amount: Some(payment.amount),
                ..Default::default()
            }),
            quantity: Some(1),
            ..Default::default()
        }]);
        create_session_params.mode = Some(CheckoutSessionMode::Payment);

        let mut metadata = Metadata::new();
        metadata.insert("payment_id".to_string(), payment.id.to_string());
        create_session_params.payment_intent_data = Some(CreateCheckoutSessionPaymentIntentData {
            metadata,
            ..Default::default()
        });

        let session = CheckoutSession::create(&self.stripe_client, create_session_params)
            .await
            .map_err(|e| e.to_string())?;

        // safe to unwrap, as this is an active session that we just created
        Ok(session.url.unwrap())
    }
}

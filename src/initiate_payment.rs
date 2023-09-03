use lambda_http::{service_fn, Body, Error, Request, Response};
use serde::{Deserialize, Serialize};
use serverless_payments::{
    payment::{Payment, PaymentStatus},
    payment_client::PaymentClient,
    payments_repository::PaymentsRepository,
    request_utils::get_body,
};
use tracing_subscriber::FmtSubscriber;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct InitiatePaymentRequest {
    pub amount: i64,
    pub sender: String,
}

#[derive(Serialize)]
pub struct InitiatePaymentResponse {
    pub redirect_url: String,
}

#[tracing::instrument]
async fn handler(event: Request) -> Result<Response<Body>, Error> {
    // Get the payment request from the event
    let payment_request: InitiatePaymentRequest = get_body(event)?;

    // generate a payment id to attach it to the webhook success uri
    let payment_id = Uuid::new_v4().to_string();
    let payment = Payment::new(
        payment_id,
        payment_request.amount,
        payment_request.sender.clone(),
        PaymentStatus::Pending,
    );

    // Send the event to Stripe
    let payment_client = PaymentClient::new();
    let redirect_url = payment_client.initiate_payment(&payment).await?;
    // Save the data in DynamoDB
    let payments_repository = PaymentsRepository::get().await;
    payments_repository.insert_payment(payment).await?;

    // Return the redirect URI
    let response = InitiatePaymentResponse { redirect_url };
    Ok(Response::new(Body::from(
        serde_json::to_string(&response).unwrap(),
    )))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .with_ansi(false)
        .without_time()
        .with_target(false)
        .init();
    lambda_http::run(service_fn(handler)).await?;
    Ok(())
}

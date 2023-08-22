use lambda_http::{service_fn, Body, Error, Request, Response};
use serverless_payments::{
    database::PaymentsRepository,
    domain::{Payment, PaymentRequest, PaymentStatus},
    payment_client::PaymentClient,
    request_utils::get_body,
};
use tracing_subscriber::FmtSubscriber;

#[derive(serde::Serialize)]
pub struct InitiatePaymentResponse {
    pub redirect_url: String,
}

#[tracing::instrument]
async fn handler(event: Request) -> Result<Response<Body>, Error> {
    // Get the payment request from the event
    let payment_request: PaymentRequest = get_body(event)?;

    // Send the event to Stripe
    let payment_client = PaymentClient::new();
    let initiate_payment = payment_client.initiate_payment(&payment_request).await?;

    // Save the data in DynamoDB
    let payments_repository = PaymentsRepository::get().await;
    let payment = Payment::new(
        initiate_payment.payment_intent_id,
        payment_request.amount,
        payment_request.sender.clone(),
        PaymentStatus::Pending,
    );
    payments_repository.insert_payment(payment).await?;

    // Return the redirect URI
    let response = InitiatePaymentResponse {
        redirect_url: initiate_payment.redirect_url,
    };
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

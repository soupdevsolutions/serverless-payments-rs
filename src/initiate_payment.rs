use lambda_http::{service_fn, Body, Error, Request, Response};
use serverless_payments::{
    database::PaymentsRepository, domain::PaymentRequest, payment_client::PaymentClient,
    request_utils::get_body,
};
use tracing_subscriber::FmtSubscriber;

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

#[tracing::instrument]
async fn handler(event: Request) -> Result<Response<Body>, Error> {
    // Get the payment request from the event
    let payment_request: PaymentRequest = get_body(event)?;

    // Send the event to Stripe
    let payment_client = PaymentClient::new();
    let redirect_url = payment_client.initiate_payment(&payment_request).await?;

    // Save the data to the database
    let payments_repository = PaymentsRepository::get().await;
    payments_repository
        .insert_payment(payment_request.into())
        .await?;

    // Return the redirect URI
    let response = InitiatePaymentResponse { redirect_url };
    Ok(Response::new(Body::from(
        serde_json::to_string(&response).unwrap(),
    )))
}

#[derive(serde::Serialize)]
pub struct InitiatePaymentResponse {
    pub redirect_url: Option<String>,
}

use aws_sdk_dynamodb::Client;
use lambda_http::{service_fn, Body, Error, Request, Response};
use serverless_payments::{
    domain::PaymentRequest, payment_client::PaymentClient, payments_repository::PaymentsRepository,
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
    let config = aws_config::load_from_env().await;
    let db_client = Client::new(&config);
    let payments_repository = PaymentsRepository::new(db_client);
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

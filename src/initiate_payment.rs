use aws_sdk_dynamodb::Client;
use lambda_http::{service_fn, Body, Error, Request, Response};
use serverless_payments::{
    domain::PaymentRequest, payment_client::PaymentClient, payments_repository::PaymentsRepository,
    request_utils::get_body,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::run(service_fn(handler)).await?;
    Ok(())
}

async fn handler(event: Request) -> Result<Response<Body>, Error> {
    // Get the payment request from the event
    let payment_request: PaymentRequest = get_body(event)?;

    // Send the event to Stripe
    let payment_client = PaymentClient::new("");
    let payment = payment_client.initiate_payment(payment_request).await?;

    // Save the data to the database
    let config = aws_config::load_from_env().await;
    let db_client = Client::new(&config);
    let payments_repository = PaymentsRepository::new(db_client, "payments");
    payments_repository.insert_payment(payment).await?;
    // Return the redirect URI

    Ok(Response::new(Body::from("Hello world!")))
}

use lambda_http::{service_fn, Body, Error, Request, Response};
use serverless_payments::{
    database::PaymentsRepository,
    domain::PaymentStatus,
    environment::{get_env_var, STRIPE_WEBHOOK_SECRET},
    request_utils::{get_header, get_query_string_parameter},
};
use stripe::Webhook;
use tracing_subscriber::FmtSubscriber;

const SIGNATURE_HEADER_KEY: &str = "Stripe-Signature";
const PAYMENT_ID_QUERY_STRING_KEY: &str = "payment_id";

async fn handler(event: Request) -> Result<Response<Body>, Error> {
    let signature = get_header(&event, SIGNATURE_HEADER_KEY)?;
    let webhook_secret = get_env_var(STRIPE_WEBHOOK_SECRET)?;
    let payment_id = get_query_string_parameter(&event, PAYMENT_ID_QUERY_STRING_KEY)?;

    let event_body = match event.body() {
        Body::Text(s) => s,
        _ => {
            tracing::error!("Error getting event body");
            return Err(Error::from("Error getting event body"));
        }
    };

    // constructing the event to validate the incoming data
    let _webhook_event = Webhook::construct_event(event_body, &signature, &webhook_secret)
        .map_err(|e| {
            tracing::error!("Error constructing webhook event: {e}");
            Error::from(format!("Error constructing webhook event: {e}"))
        })?;

    let payment_repository = PaymentsRepository::get().await;
    payment_repository
        .update_payment_status(&payment_id, PaymentStatus::Completed)
        .await?;

    Ok(Response::new(Body::from(())))
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

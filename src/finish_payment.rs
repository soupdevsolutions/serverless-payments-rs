use lambda_http::{service_fn, Body, Error, Request, Response};
use serverless_payments::{
    database::PaymentsRepository,
    domain::PaymentStatus,
    environment::{get_env_var, STRIPE_SECRET_KEY},
    request_utils::get_header,
};
use stripe::{EventObject, Webhook};
use tracing_subscriber::FmtSubscriber;

const SIGNATURE_HEADER_KEY: &str = "Stripe-Signature";

async fn handler(event: Request) -> Result<Response<Body>, Error> {
    let signature = get_header(&event, SIGNATURE_HEADER_KEY)?;
    let secret_key = get_env_var(STRIPE_SECRET_KEY)?;
    let event_body = match event.body() {
        Body::Text(s) => s,
        _ => {
            tracing::error!("Error getting event body");
            return Err(Error::from("Error getting event body"));
        }
    };

    let webhook_event =
        Webhook::construct_event(event_body, &signature, &secret_key).map_err(|_| {
            tracing::error!("Error constructing webhook event");
            Error::from("Error constructing webhook event")
        })?;

    let payment_intent_id = {
        let payment_intent = match webhook_event.data.object {
            // safe to unwrap, as this charge is the result of a PaymentIntent confirmation
            EventObject::Charge(charge) => charge.payment_intent.unwrap(),
            _ => {
                tracing::error!("Error getting payment intent");
                return Err(Error::from("Error getting payment intent"));
            }
        };

        payment_intent.id()
    };

    let payment_repository = PaymentsRepository::get().await;
    payment_repository
        .update_payment_status(payment_intent_id.as_str(), PaymentStatus::Completed)
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

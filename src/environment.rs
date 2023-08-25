// Generic variables
pub const DOMAIN: &str = "DOMAIN";

// DynamoDB tables
pub const PAYMENTS_TABLE: &str = "PAYMENTS_TABLE_NAME";

// Stripe
pub const STRIPE_SECRET_KEY: &str = "STRIPE_SECRET_KEY";
pub const STRIPE_WEBHOOK_SECRET: &str = "STRIPE_WEBHOOK_SECRET";

pub fn get_env_var(key: &str) -> Result<String, String> {
    match std::env::var(key) {
        Ok(val) => Ok(val),
        Err(e) => Err(format!("{} is not set. Internal error: {}", key, e)),
    }
}

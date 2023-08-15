use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentRequest {
    pub amount: i64,
    pub sender: String,
}

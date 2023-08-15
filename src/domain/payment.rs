use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::PaymentRequest;

#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentStatus {
    Pending,
    Failed,
    Completed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
    pub id: String,
    pub amount: i64,
    pub sender: String,
    pub status: PaymentStatus,
}

impl Payment {
    pub fn new(amount: i64, sender: String, status: PaymentStatus) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            amount,
            sender,
            status,
        }
    }
}

impl From<PaymentRequest> for Payment {
    fn from(request: PaymentRequest) -> Self {
        Self::new(request.amount, request.sender, PaymentStatus::Pending)
    }
}

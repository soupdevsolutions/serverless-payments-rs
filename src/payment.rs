use serde::{Deserialize, Serialize};

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
    pub fn new(id: String, amount: i64, sender: String, status: PaymentStatus) -> Self {
        Self {
            id,
            amount,
            sender,
            status,
        }
    }
}

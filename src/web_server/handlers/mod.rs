pub mod users;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetailsResponse {
    pub error_id: String,
    pub error_message: String
}
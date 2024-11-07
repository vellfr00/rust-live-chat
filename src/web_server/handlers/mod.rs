pub mod users;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct ErrorDetailsResponse {
    error_id: String,
    error_message: String
}
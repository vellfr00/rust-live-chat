pub mod users;
pub mod rooms;

use serde::{Serialize, Deserialize};
use warp::http::StatusCode;
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use crate::entities::server::Server;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetailsResponse {
    pub error_id: String,
    pub error_message: String
}

pub async fn is_server_reachable(server: Arc<Mutex<Server>>) -> Result<impl warp::Reply, Infallible> {
    let server = server.lock();
    if server.is_err() {
        let error_object = &ErrorDetailsResponse {
            error_id: "SERVER_UNREACHABLE".to_string(),
            error_message: "The server is unreachable".to_string()
        };
        return Ok(warp::reply::with_status(warp::reply::json(&serde_json::json!(error_object)), StatusCode::INTERNAL_SERVER_ERROR));
    }

    Ok(warp::reply::with_status(warp::reply::json(&serde_json::json!({"status": "OK__SERVER_REACHABLE"})), StatusCode::OK))
}
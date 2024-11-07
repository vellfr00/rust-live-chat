use crate::entities::server::Server;
use std::sync::{Arc, Mutex};
use warp::http::StatusCode;
use std::convert::Infallible;

pub async fn user_exists_in_server_by_username(username: String, server: Arc<Mutex<Server>>) -> Result<impl warp::Reply, Infallible> {
    let server = server.lock().unwrap();
    let user = server.get_user_by_username(&username);
    match user {
        Some(_) => Ok(StatusCode::OK),
        None => Ok(StatusCode::NOT_FOUND)
    }
}

pub async fn register_user_to_server(username: String, server: Arc<Mutex<Server>>) -> Result<impl warp::Reply, Infallible> {
    let mut server = server.lock().unwrap();
    match server.register_user(&username) {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Ok(StatusCode::BAD_REQUEST)
    }
}
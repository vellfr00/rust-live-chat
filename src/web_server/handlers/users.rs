use crate::entities::server::Server;
use super::ErrorDetailsResponse;
use std::sync::{Arc, Mutex};
use warp::http::StatusCode;
use std::convert::Infallible;

pub async fn get_user_in_server_by_username(username: String, server: Arc<Mutex<Server>>) -> Result<impl warp::Reply, Infallible> {
    let server = server.lock().unwrap();
    let user = server.get_user_by_username(&username);
    match user {
        Some(user_arc) => {
            let user = user_arc.clone();
            let json_response = warp::reply::json(&*user);
            Ok(warp::reply::with_status(json_response, StatusCode::OK))
        },
        None => {
            let json_response = warp::reply::json(&ErrorDetailsResponse {
                error_id: "ERR__USER_NOT_FOUND".to_string(),
                error_message: format!("User with username {} not found in server", username)
            });
            Ok(warp::reply::with_status(json_response, StatusCode::NOT_FOUND))
        }
    }
}

pub async fn register_user_to_server(username: String, server: Arc<Mutex<Server>>) -> Result<impl warp::Reply, Infallible> {
    let mut server = server.lock().unwrap();
    match server.register_user(&username) {
        Ok(_) => {
            let user = server.get_user_by_username(&username).unwrap();
            let json_response = warp::reply::json(&*user);
            Ok(warp::reply::with_status(json_response, StatusCode::CREATED))
        },
        Err(_) => {
            let json_response = warp::reply::json(&ErrorDetailsResponse {
                error_id: "ERR__USER_ALREADY_EXISTS".to_string(),
                error_message: format!("User with username {} already exists in server", username)
            });
            Ok(warp::reply::with_status(json_response, StatusCode::CONFLICT))
        }
    }
}
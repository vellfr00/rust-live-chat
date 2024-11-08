use crate::entities::server::Server;
use super::ErrorDetailsResponse;
use std::{collections::HashMap, sync::{Arc, Mutex}};
use warp::http::StatusCode;
use std::convert::Infallible;

pub async fn get_room_by_name(room_name: String, server: Arc<Mutex<Server>>) -> Result<impl warp::Reply, Infallible> {
    let server = server.lock().unwrap();
    let room = server.get_room_by_name(&room_name);
    match room {
        Some(room_arc) => {
            let room = room_arc.lock().unwrap();
            let room_summary = serde_json::json!({
                "id": room.id,
                "name": room.name,
                "users": room.users
            });
            let json_response = warp::reply::json(&room_summary);
            Ok(warp::reply::with_status(json_response, StatusCode::OK))
        }
        None => {
            let json_response = warp::reply::json(&ErrorDetailsResponse {
                error_id: "ERR__ROOM_NOT_FOUND".to_string(),
                error_message: format!("Room with name {} not found in server", room_name)
            });
            Ok(warp::reply::with_status(json_response, StatusCode::NOT_FOUND))
        }
    }
}

pub async fn get_user_in_room_by_username(room_name: String, username: String, server: Arc<Mutex<Server>>) -> Result<impl warp::Reply, Infallible> {
    let server = server.lock().unwrap();
    let room = server.get_room_by_name(&room_name);
    match room {
        Some(room_arc) => {
            match server.get_user_by_username(&username) {
                Some(user_arc) => {
                    let user = user_arc.clone();
                    let room = room_arc.lock().unwrap();
                    match room.is_user_in_room(user.clone()) {
                        true => {
                            let json_response = warp::reply::json(&*user);
                            Ok(warp::reply::with_status(json_response, StatusCode::OK))
                        },
                        false => {
                            let json_response = warp::reply::json(&ErrorDetailsResponse {
                                error_id: "ERR__USER_NOT_IN_ROOM".to_string(),
                                error_message: format!("User with username {} is not in room with name {}", username, room_name)
                            });
                            Ok(warp::reply::with_status(json_response, StatusCode::NOT_FOUND))
                        }
                    }
                }
                None => {
                    let json_response = warp::reply::json(&ErrorDetailsResponse {
                        error_id: "ERR__USER_NOT_FOUND".to_string(),
                        error_message: format!("User with username {} not found in server", username)
                    });
                    Ok(warp::reply::with_status(json_response, StatusCode::NOT_FOUND))
                }
            }
        }
        None => {
            let json_response = warp::reply::json(&ErrorDetailsResponse {
                error_id: "ERR__ROOM_NOT_FOUND".to_string(),
                error_message: format!("Room with name {} not found in server", room_name)
            });
            Ok(warp::reply::with_status(json_response, StatusCode::NOT_FOUND))
        }
    }
}

pub async fn create_room(room_name: String, query_params: HashMap<String, String>, server: Arc<Mutex<Server>>) -> Result<impl warp::Reply, Infallible> {
    let creator_username = query_params.get("creator_username");
    if creator_username.is_none() {
        let json_response = warp::reply::json(&ErrorDetailsResponse {
            error_id: "ERR__ROOM_CREATE_BAD_REQUEST".to_string(),
            error_message: "Missing creator_username query parameter".to_string()
        });
        return Ok(warp::reply::with_status(json_response, StatusCode::BAD_REQUEST));
    }
    let creator_username = creator_username.unwrap();

    let mut server = server.lock().unwrap();
    match server.create_room(&room_name, &creator_username) {
        Ok(_) => {
            let room = server.get_room_by_name(&room_name).unwrap();
            let room = room.lock().unwrap();
            let room_summary = serde_json::json!({
                "id": room.id,
                "name": room.name,
                "users": room.users
            });
            let json_response = warp::reply::json(&room_summary);
            Ok(warp::reply::with_status(json_response, StatusCode::CREATED))
        },
        Err(err_message) => {
            let json_response = warp::reply::json(&ErrorDetailsResponse {
                error_id: "ERR__ROOM_CREATE_CONFLICT".to_string(),
                error_message: format!("Cannot create room {}: {}", creator_username, err_message)
            });
            Ok(warp::reply::with_status(json_response, StatusCode::CONFLICT))
        }
    }
}

pub async fn add_user_to_room(room_name: String, username: String, server: Arc<Mutex<Server>>) -> Result<impl warp::Reply, Infallible> {
    let mut server = server.lock().unwrap();
    match server.add_user_to_room(&room_name, &username) {
        Ok(_) => {
            let room = server.get_room_by_name(&room_name).unwrap();
            let room = room.lock().unwrap();
            let room_summary = serde_json::json!({
                "id": room.id,
                "name": room.name,
                "users": room.users
            });
            let json_response = warp::reply::json(&room_summary);
            Ok(warp::reply::with_status(json_response, StatusCode::CREATED))
        },
        Err(err_message) => {
            let json_response = warp::reply::json(&ErrorDetailsResponse {
                error_id: "ERR__USER_ADD_TO_ROOM_CONFLICT".to_string(),
                error_message: format!("Cannot add user {} to room {}: {}", username, room_name, err_message)
            });
            Ok(warp::reply::with_status(json_response, StatusCode::CONFLICT))
        }
    }
}

pub async fn get_room_messages(room_name: String, server: Arc<Mutex<Server>>) -> Result<impl warp::Reply, Infallible> {
    let server = server.lock().unwrap();
    match server.get_room_messages(&room_name) {
        Ok(messages) => {
            let json_response = warp::reply::json(&messages);
            Ok(warp::reply::with_status(json_response, StatusCode::OK))
        },
        Err(err_message) => {
            let json_response = warp::reply::json(&ErrorDetailsResponse {
                error_id: "ERR__ROOM_MESSAGES_CONFLICT".to_string(),
                error_message: format!("Cannot get messages for room {}: {}", room_name, err_message)
            });
            Ok(warp::reply::with_status(json_response, StatusCode::CONFLICT))
        }
    }
}

pub async fn post_message_to_room(room_name: String, body: HashMap<String, String>, server: Arc<Mutex<Server>>) -> Result<impl warp::Reply, Infallible> {
    let username = body.get("username");
    let message = body.get("message");
    if username.is_none() || message.is_none() {
        let json_response = warp::reply::json(&ErrorDetailsResponse {
            error_id: "ERR__MESSAGE_POST_TO_ROOM_BAD_REQUEST".to_string(),
            error_message: "Missing username or message in request body".to_string()
        });
        return Ok(warp::reply::with_status(json_response, StatusCode::BAD_REQUEST));
    }
    let username = username.unwrap();
    let message = message.unwrap();

    let mut server = server.lock().unwrap();
    match server.post_message_to_room(&room_name, &username, &message) {
        Ok(_) => {
            let room = server.get_room_by_name(&room_name).unwrap();
            let room = room.lock().unwrap();
            let room_summary = serde_json::json!({
                "id": room.id,
                "name": room.name,
                "users": room.users
            });
            let json_response = warp::reply::json(&room_summary);
            Ok(warp::reply::with_status(json_response, StatusCode::CREATED))
        },
        Err(err_message) => {
            let json_response = warp::reply::json(&ErrorDetailsResponse {
                error_id: "ERR__MESSAGE_POST_TO_ROOM_CONFLICT".to_string(),
                error_message: format!("Cannot post message to room {}: {}", room_name, err_message)
            });
            Ok(warp::reply::with_status(json_response, StatusCode::CONFLICT))
        }
    }
}
use crate::web_server::handlers;
use crate::entities::server::Server;
use std::{collections::HashMap, sync::{Arc, Mutex}};
use warp::Filter;
use super::with_server;

pub fn rooms_routes(server: Arc<Mutex<Server>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_room_by_name(server.clone())
        .or(get_user_in_room_by_name(server.clone()))
        .or(create_room(server.clone()))
        .or(add_user_to_room(server.clone()))
}

/**
 * GET /rooms/:room_name
 * Checks if a room exists in the server and returns it.
 * Returns 200 OK if the room exists in the server, 404 NOT FOUND otherwise.
 */
fn get_room_by_name(server: Arc<Mutex<Server>>) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("rooms" / String)
        .and(warp::get())
        .and(with_server(server))
        .and_then(handlers::rooms::get_room_by_name)
}

/**
 * GET /rooms/:room_name/users/:username
 * Checks if a user exists in a room and returns it.
 * Returns 200 OK if the user exists in the room, 404 NOT FOUND otherwise.
 */
fn get_user_in_room_by_name(server: Arc<Mutex<Server>>) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("rooms" / String / "users" / String)
        .and(warp::get())
        .and(with_server(server))
        .and_then(handlers::rooms::get_user_in_room_by_username)
}

/**
 * POST /rooms/:room_name?creator_username=:username
 * Creates a new room in the server and adds the creator user to it.
 * Returns 201 CREATED if the room was successfully created, 409 CONFLICT if a conflict occurs.
 * If missing query parameter, returns 400 BAD REQUEST.
 */
fn create_room(server: Arc<Mutex<Server>>) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("rooms" / String)
        .and(warp::post())
        .and(warp::query::<HashMap<String, String>>())
        .and(with_server(server))
        .and_then(handlers::rooms::create_room)
}

/**
 * POST /rooms/:room_name/users/:username
 * Adds a user to a room in the server.
 * Returns 200 OK if the user was successfully added to the room, 409 CONFLICT if a conflict occurs.
 */
fn add_user_to_room(server: Arc<Mutex<Server>>) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("rooms" / String / "users" / String)
        .and(warp::post())
        .and(with_server(server))
        .and_then(handlers::rooms::add_user_to_room)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::user::User;
    use crate::entities::room::Room;
    use crate::entities::server::Server;
    use warp::{http::StatusCode, reply::Json};
    use serde_json::{self};
    use warp::test::request;
    use crate::web_server::handlers::ErrorDetailsResponse;

    #[tokio::test]
    async fn test_get_room_by_name() {
        let server = Arc::new(Mutex::new(Server::new("test_server".to_string())));
        server.clone().lock().unwrap().register_user("test_user").unwrap();
        server.clone().lock().unwrap().create_room("test_room", "test_user").unwrap();

        let response = request()
            .method("GET")
            .path("/rooms/test_room")
            .reply(&rooms_routes(server.clone()))
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        assert_ne!(response.body().len(), 0);

        let response_string: String = String::from_utf8(response.body().to_vec()).unwrap();
        assert!(response_string.contains("test_room"));
    }

    #[tokio::test]
    async fn test_get_room_by_name_room_not_found() {
        let server = Arc::new(Mutex::new(Server::new("test_server".to_string())));

        let response = request()
            .method("GET")
            .path("/rooms/test_room")
            .reply(&rooms_routes(server.clone()))
            .await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert_ne!(response.body().len(), 0);
        
        let error: ErrorDetailsResponse = serde_json::from_slice(response.body()).unwrap();
        assert_eq!(error.error_id, "ERR__ROOM_NOT_FOUND");
    }

    #[tokio::test]
    async fn test_get_user_in_room_by_name() {
        let server = Arc::new(Mutex::new(Server::new("test_server".to_string())));
        server.clone().lock().unwrap().register_user("test_user").unwrap();
        server.clone().lock().unwrap().create_room("test_room", "test_user").unwrap();

        let response = request()
            .method("GET")
            .path("/rooms/test_room/users/test_user")
            .reply(&rooms_routes(server.clone()))
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        assert_ne!(response.body().len(), 0);

        let user: User = serde_json::from_slice(response.body()).unwrap();
        assert_eq!(user.username, "test_user");
    }

    #[tokio::test]
    async fn test_get_user_in_room_by_name_room_not_found() {
        let server = Arc::new(Mutex::new(Server::new("test_server".to_string())));
        server.clone().lock().unwrap().register_user("test_user").unwrap();

        let response = request()
            .method("GET")
            .path("/rooms/test_room/users/test_user")
            .reply(&rooms_routes(server.clone()))
            .await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert_ne!(response.body().len(), 0);
        
        let error: ErrorDetailsResponse = serde_json::from_slice(response.body()).unwrap();
        assert_eq!(error.error_id, "ERR__ROOM_NOT_FOUND");
    }

    #[tokio::test]
    async fn test_get_user_in_room_by_name_user_not_found() {
        let server = Arc::new(Mutex::new(Server::new("test_server".to_string())));
        server.clone().lock().unwrap().register_user("test_user").unwrap();
        server.clone().lock().unwrap().create_room("test_room", "test_user").unwrap();

        let response = request()
            .method("GET")
            .path("/rooms/test_room/users/test_user2")
            .reply(&rooms_routes(server.clone()))
            .await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert_ne!(response.body().len(), 0);
        
        let error: ErrorDetailsResponse = serde_json::from_slice(response.body()).unwrap();
        assert_eq!(error.error_id, "ERR__USER_NOT_FOUND");
    }

    #[tokio::test]
    async fn test_create_room() {
        let server = Arc::new(Mutex::new(Server::new("test_server".to_string())));
        server.clone().lock().unwrap().register_user("test_user").unwrap();

        let response = request()
            .method("POST")
            .path("/rooms/test_room?creator_username=test_user")
            .reply(&rooms_routes(server.clone()))
            .await;

        assert_eq!(response.status(), StatusCode::CREATED);
        assert_ne!(response.body().len(), 0);

        let response_string = String::from_utf8(response.body().to_vec()).unwrap();
        assert!(response_string.contains("test_room"));
    }

    #[tokio::test]
    async fn test_create_room_missing_query_param() {
        let server = Arc::new(Mutex::new(Server::new("test_server".to_string())));
        server.clone().lock().unwrap().register_user("test_user").unwrap();

        let response = request()
            .method("POST")
            .path("/rooms/test_room")
            .reply(&rooms_routes(server.clone()))
            .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert_ne!(response.body().len(), 0);
        
        let error: ErrorDetailsResponse = serde_json::from_slice(response.body()).unwrap();
        assert_eq!(error.error_id, "ERR__ROOM_CREATE_BAD_REQUEST");
    }

    #[tokio::test]
    async fn test_create_room_user_not_found() {
        let server = Arc::new(Mutex::new(Server::new("test_server".to_string())));

        let response = request()
            .method("POST")
            .path("/rooms/test_room?creator_username=test_user")
            .reply(&rooms_routes(server.clone()))
            .await;

        assert_eq!(response.status(), StatusCode::CONFLICT);
        assert_ne!(response.body().len(), 0);
        
        let error: ErrorDetailsResponse = serde_json::from_slice(response.body()).unwrap();
        assert_eq!(error.error_id, "ERR__ROOM_CREATE_CONFLICT");
    }

    #[tokio::test]
    async fn test_create_room_room_already_exists() {
        let server = Arc::new(Mutex::new(Server::new("test_server".to_string())));
        server.clone().lock().unwrap().register_user("test_user").unwrap();
        server.clone().lock().unwrap().create_room("test_room", "test_user").unwrap();

        let response = request()
            .method("POST")
            .path("/rooms/test_room?creator_username=test_user")
            .reply(&rooms_routes(server.clone()))
            .await;

        assert_eq!(response.status(), StatusCode::CONFLICT);
        assert_ne!(response.body().len(), 0);
        
        let error: ErrorDetailsResponse = serde_json::from_slice(response.body()).unwrap();
        assert_eq!(error.error_id, "ERR__ROOM_CREATE_CONFLICT");
    }

    #[tokio::test]
    async fn test_add_user_to_room() {
        let server = Arc::new(Mutex::new(Server::new("test_server".to_string())));
        server.clone().lock().unwrap().register_user("test_user").unwrap();
        server.clone().lock().unwrap().create_room("test_room", "test_user").unwrap();
        server.clone().lock().unwrap().register_user("test_user2").unwrap();

        let response = request()
            .method("POST")
            .path("/rooms/test_room/users/test_user2")
            .reply(&rooms_routes(server.clone()))
            .await;

        assert_eq!(response.status(), StatusCode::CREATED);
        assert_ne!(response.body().len(), 0);

        let response_string = String::from_utf8(response.body().to_vec()).unwrap();
        assert!(response_string.contains("test_user2"));
    }

    #[tokio::test]
    async fn test_add_user_to_room_room_not_found() {
        let server = Arc::new(Mutex::new(Server::new("test_server".to_string())));
        server.clone().lock().unwrap().register_user("test_user").unwrap();

        let response = request()
            .method("POST")
            .path("/rooms/test_room/users/test_user")
            .reply(&rooms_routes(server.clone()))
            .await;

        assert_eq!(response.status(), StatusCode::CONFLICT);
        assert_ne!(response.body().len(), 0);
        
        let error: ErrorDetailsResponse = serde_json::from_slice(response.body()).unwrap();
        assert_eq!(error.error_id, "ERR__USER_ADD_TO_ROOM_CONFLICT");
    }

    #[tokio::test]
    async fn test_add_user_to_room_user_not_found() {
        let server = Arc::new(Mutex::new(Server::new("test_server".to_string())));
        server.clone().lock().unwrap().register_user("test_user").unwrap();
        server.clone().lock().unwrap().create_room("test_room", "test_user").unwrap();

        let response = request()
            .method("POST")
            .path("/rooms/test_room/users/test_user2")
            .reply(&rooms_routes(server.clone()))
            .await;

        assert_eq!(response.status(), StatusCode::CONFLICT);
        assert_ne!(response.body().len(), 0);
        
        let error: ErrorDetailsResponse = serde_json::from_slice(response.body()).unwrap();
        assert_eq!(error.error_id, "ERR__USER_ADD_TO_ROOM_CONFLICT");
    }

    #[tokio::test]
    async fn test_add_user_to_room_user_already_in_room() {
        let server = Arc::new(Mutex::new(Server::new("test_server".to_string())));
        server.clone().lock().unwrap().register_user("test_user").unwrap();
        server.clone().lock().unwrap().create_room("test_room", "test_user").unwrap();

        let response = request()
            .method("POST")
            .path("/rooms/test_room/users/test_user")
            .reply(&rooms_routes(server.clone()))
            .await;

        assert_eq!(response.status(), StatusCode::CONFLICT);
        assert_ne!(response.body().len(), 0);
        
        let error: ErrorDetailsResponse = serde_json::from_slice(response.body()).unwrap();
        assert_eq!(error.error_id, "ERR__USER_ADD_TO_ROOM_CONFLICT");
    }
}
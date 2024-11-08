use crate::entities::server::Server;
use crate::web_server::handlers;
use std::sync::{Arc, Mutex};
use warp::Filter;
use super::with_server;

pub fn users_routes(server: Arc<Mutex<Server>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_user_in_server_by_username(server.clone())
        .or(register_user_to_server(server.clone()))
}

/**
 * GET /users/:username
 * Checks if a user exists in the server and returns it.
 * Returns 200 OK if the user exists in the server, 404 NOT FOUND otherwise.
 */
fn get_user_in_server_by_username(server: Arc<Mutex<Server>>) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("users" / String)
        .and(warp::get())   
        .and(with_server(server))
        .and_then(handlers::users::get_user_in_server_by_username)
}

/**
 * POST /users/:username
 * Registers a new user to the server.
 * Returns 201 CREATED if the user was successfully registered, 400 BAD REQUEST otherwise.
 */
fn register_user_to_server(server: Arc<Mutex<Server>>) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("users" / String)
        .and(warp::post())   
        .and(with_server(server))
        .and_then(handlers::users::register_user_to_server)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::user::User;
    use crate::entities::server::Server;
    use warp::http::StatusCode;
    use serde_json::{self};
    use warp::test::request;
    use crate::web_server::handlers::ErrorDetailsResponse;


    #[tokio::test]
    async fn test_get_user_in_server_by_username() {
        let server = Arc::new(Mutex::new(Server::new()));
        server.clone().lock().unwrap().register_user("test_user").unwrap();

        let response = request()
            .method("GET")
            .path("/users/test_user")
            .reply(&users_routes(server.clone()))
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        assert_ne!(response.body().len(), 0);
        
        let user: User = serde_json::from_slice(response.body()).unwrap();
        assert_eq!(user.username, "test_user");
    }

    #[tokio::test]
    async fn test_get_user_in_server_by_username_not_found() {
        let server = Arc::new(Mutex::new(Server::new()));

        let response = request()
            .method("GET")
            .path("/users/test_user")
            .reply(&users_routes(server.clone()))
            .await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let error_response = &ErrorDetailsResponse {
            error_id: "ERR__USER_NOT_FOUND".to_string(),
            error_message: "User with username test_user not found in server".to_string()
        };
        let error_response_to_serialized_string = serde_json::to_string(&error_response).unwrap();
        let response_body = std::str::from_utf8(response.body()).unwrap();
        assert_eq!(response_body, error_response_to_serialized_string);
    }

    #[tokio::test]
    async fn test_register_user_to_server() {
        let server = Arc::new(Mutex::new(Server::new()));

        let response = request()
            .method("POST")
            .path("/users/test_user")
            .reply(&users_routes(server.clone()))
            .await;

        assert_eq!(response.status(), StatusCode::CREATED);
        assert_ne!(response.body().len(), 0);

        let user: User = serde_json::from_slice(response.body()).unwrap();
        assert_eq!(user.username, "test_user");
    }

    #[tokio::test]
    async fn test_register_user_to_server_already_exists() {
        let server = Arc::new(Mutex::new(Server::new()));
        let user = User::new("test_user".to_string());
        server.lock().unwrap().register_user(&user.username).unwrap();

        let response = request()
            .method("POST")
            .path("/users/test_user")
            .reply(&users_routes(server.clone()))
            .await;

        assert_eq!(response.status(), StatusCode::CONFLICT);

        let error_response = &ErrorDetailsResponse {
            error_id: "ERR__USER_ALREADY_EXISTS".to_string(),
            error_message: "User with username test_user already exists in server".to_string()
        };
        let error_response_to_serialized_string = serde_json::to_string(&error_response).unwrap();
        let response_body = std::str::from_utf8(response.body()).unwrap();
        assert_eq!(response_body, error_response_to_serialized_string);
    }
}
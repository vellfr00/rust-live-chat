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
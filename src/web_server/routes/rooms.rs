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
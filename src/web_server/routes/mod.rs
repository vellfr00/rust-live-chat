pub mod users;

use crate::entities::server::Server;
use std::sync::{Arc, Mutex};
use warp::Filter;

pub fn routes(server: Arc<Mutex<Server>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    users::users_routes(server.clone())
}

fn with_server(server: Arc<Mutex<Server>>) -> impl Filter<Extract = (Arc<Mutex<Server>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || server.clone())
}
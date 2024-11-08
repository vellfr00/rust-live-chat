pub mod users;
pub mod rooms;

use crate::entities::server::Server;
use std::sync::{Arc, Mutex};
use warp::Filter;

use super::handlers;

pub fn routes(server: Arc<Mutex<Server>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    is_server_reachable_route(server.clone())
        .or(users::users_routes(server.clone()))
        .or(rooms::rooms_routes(server.clone()))
}

/**
 * GET /status
 * Returns 200 OK if the server is reachable, with a JSON response body
 * If there is a problem with unwrapping server reference, returns 500 Internal Server Error
 */
fn is_server_reachable_route(server: Arc<Mutex<Server>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("status")
        .and(warp::get())
        .and(with_server(server.clone()))
        .and_then(handlers::is_server_reachable)
}

fn with_server(server: Arc<Mutex<Server>>) -> impl Filter<Extract = (Arc<Mutex<Server>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || server.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use warp::http::StatusCode;

    #[tokio::test]
    async fn test_is_server_reachable_route() {
        let server = Arc::new(Mutex::new(Server::new()));
        let route = is_server_reachable_route(server.clone());

        let response = warp::test::request()
            .method("GET")
            .path("/status")
            .reply(&route)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }
}
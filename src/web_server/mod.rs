pub mod handlers;
pub mod routes;

use crate::entities::server::Server;
use std::sync::{Arc, Mutex};

pub struct WebServer {
    pub server: Arc<Mutex<Server>>
}

impl WebServer {
    pub fn new() -> WebServer {
        WebServer {
            server: Arc::new(Mutex::new(Server::new()))
        }
    }

    pub async fn run(&self) {
        let address = [127, 0, 0, 1];
        let port = 3030;

        println!("{}", format!("Starting server at http://{}.{}.{}.{}:{}/", address[0], address[1], address[2], address[3], port));

        warp::serve(routes::routes(self.server.clone()))
            .run((address, port))
            .await;
    }
}
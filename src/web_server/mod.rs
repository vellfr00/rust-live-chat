pub mod handlers;
pub mod routes;

use crate::entities::server::Server;
use std::{env, sync::{Arc, Mutex}};

pub struct WebServer {
    host: [u8; 4],
    port: u16,
    pub server: Arc<Mutex<Server>>
}

impl WebServer {
    pub fn new(host: &str, port: u16) -> WebServer {
        let host_vec = host.split('.').map(|x| x.parse().unwrap()).collect::<Vec<u8>>();
        let mut host_array: [u8; 4] = host_vec.try_into().expect("Host should have 4 octets");

        match env::var("ENVIRONMENT") {
            Ok(env) => {
                match env.as_str() {
                    "docker" => {
                        println!("Running in Docker environment, using default host 0.0.0.0 to expose the server to the outside");
                        host_array = [0, 0, 0, 0];
                    },
                    _ => {
                        println!("Running in unknown environment, using provided host and port");
                    }
                }
            },
            Err(_) => { 
                print!("No ENVIRONMENT variable set, using provided host and port");
            }
        }

        WebServer {
            host: host_array,
            port,
            server: Arc::new(Mutex::new(Server::new()))
        }
    }

    pub async fn run(&self) {
        println!("{}", format!("Starting server at http://{}.{}.{}.{}:{}/", self.host[0], self.host[1], self.host[2], self.host[3], self.port));

        warp::serve(routes::routes(self.server.clone()))
            .run((self.host, self.port))
            .await;
    }
}
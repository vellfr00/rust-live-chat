use std::process::exit;

mod api;
mod flows;

pub struct CliClient {
    server_endpoint: String,
    current_username: String,
    current_room: String
}

impl CliClient {
    pub fn new(server_host: String, server_port: u16) -> CliClient {
        CliClient {
            server_endpoint: format!("http://{}:{}", server_host, server_port),
            current_username: String::new(),
            current_room: String::new()
        }
    }

    pub async fn run(&mut self) {
        println!("Starting CLI client");
        self.is_server_alive().await;
        self.authenticate_user().await;
    }

    async fn is_server_alive(&self) -> () {
        println!("Checking connection with server at {}", self.server_endpoint);
        let is_alive = api::fetch_api_is_server_alive(&self.server_endpoint).await;
        if is_alive {
            println!("Server is reachable, connection established");
        } else {
            println!("Server is unreachable, connection failed");
            exit(1);
        }
    }

    async fn authenticate_user(&mut self) {
        self.current_username = flows::user_authentication::loop_user_authentication_flow(&self.server_endpoint).await;
        println!("Authenticated as {}", self.current_username);
    }
}
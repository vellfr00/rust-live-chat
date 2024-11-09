mod entities;
mod web_server;
mod cli_client;

use clap::{Parser, ValueEnum};
use regex::Regex;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum RunType {
    /// Run the server
    Server,
    /// Run the client
    Client
}
const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 3030;

#[derive(Parser)]
struct Cli {
    /// Whether to run the server or the client
    #[arg(long)]
    run: RunType,
    /// The host where server will run (if server) or the host to connect to (if client)
    #[arg(long, value_parser = host_is_valid, default_value_t = DEFAULT_HOST.to_string())]
    host: String,
    /// The port where server will run (if server) or the port to connect to (if client)
    #[arg(long, default_value_t = DEFAULT_PORT)]
    port: u16,
}

const HOST_REGEX: &str = r"^(\d{1,3}\.){3}\d{1,3}$";
fn host_is_valid(s: &str) -> Result<String, String> {
    if Regex::new(HOST_REGEX).unwrap().is_match(s) {
        Ok(s.to_string())
    } else {
        Err("Invalid host".to_string())
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let host = cli.host;
    let port = cli.port;

    match cli.run {
        RunType::Server => {
            let web_server = web_server::WebServer::new(&host, port);
            web_server.run().await;
        },
        RunType::Client => {
            let mut cli_client = cli_client::CliClient::new(host, port);
            cli_client.run().await;
        }
    }
}

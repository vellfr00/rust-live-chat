mod entities;
mod web_server;

#[tokio::main]
async fn main() {
    let web_server = web_server::WebServer::new();
    web_server.run().await;
}

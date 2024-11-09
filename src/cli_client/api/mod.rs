pub mod users;

pub async fn fetch_api_is_server_alive(server_endpoint: &str) -> bool {
    let response = reqwest::get(&format!("{}/status", server_endpoint)).await;
    match response {
        Ok(response) => response.status().is_success(),
        Err(_) => false
    }
}
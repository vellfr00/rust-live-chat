use crate::web_server::handlers::ErrorDetailsResponse;

pub async fn fetch_api_get_user_in_server_by_username(server_endpoint: &str, username: &str) -> Result<(), ErrorDetailsResponse> {
    let response = reqwest::get(&format!("{}/users/{}", server_endpoint, username)).await;
    match response {
        Ok(response) => {
            if response.status().is_success() {
                Ok(())
            } else {
                let error_details: ErrorDetailsResponse = response.json().await.unwrap();
                Err(error_details)
            }
        }
        Err(error) => {
            Err(ErrorDetailsResponse {
                error_id: "ERR__CLIENT_FETCH_API".to_string(),
                error_message: format!("Failed to fetch API: {}", error.to_string())
            })
        }
    }
}

pub async fn fetch_api_register_user_to_server(server_endpoint: &str, username: &str) -> Result<(), ErrorDetailsResponse> {
    let response = reqwest::Client::new()
        .post(&format!("{}/users/{}", server_endpoint, username))
        .send()
        .await;
    match response {
        Ok(response) => {
            if response.status().is_success() {
                Ok(())
            } else {
                let error_details: ErrorDetailsResponse = response.json().await.unwrap();
                Err(error_details)
            }
        }
        Err(error) => {
            Err(ErrorDetailsResponse {
                error_id: "ERR__CLIENT_FETCH_API".to_string(),
                error_message: format!("Failed to fetch API: {}", error.to_string())
            })
        }
    }
}
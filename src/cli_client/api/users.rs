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

#[cfg(test)]
mod tests {
    use super::*;
    use mockito;

    #[tokio::test]
    async fn test_fetch_api_get_user_in_server_by_username() {
        let mut server = mockito::Server::new_async().await;
        let mock = server.mock("GET", "/users/test_user")
            .with_status(200)
            .create_async().await;

        let response = fetch_api_get_user_in_server_by_username(&server.url(), "test_user").await;
        assert!(response.is_ok());
        mock.assert();
    }

    #[tokio::test]
    async fn test_fetch_api_get_user_in_server_by_username_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server.mock("GET", "/users/test_user")
            .with_status(404)
            .with_body(r#"{"error_id":"ERR__USER_NOT_FOUND","error_message":"User not found"}"#)
            .create_async().await;

        let response = fetch_api_get_user_in_server_by_username(&server.url(), "test_user").await;
        assert!(response.is_err());
        assert!(response.err().unwrap().error_id == "ERR__USER_NOT_FOUND");
        mock.assert();
    }

    #[tokio::test]
    async fn test_fetch_api_get_user_in_server_by_username_error_fetch() {
        let response = fetch_api_get_user_in_server_by_username("http://localhost-non-existent:3012", "test_user").await;
        assert!(response.is_err());
        assert!(response.err().unwrap().error_id == "ERR__CLIENT_FETCH_API");
    }

    #[tokio::test]
    async fn test_fetch_api_register_user_to_server() {
        let mut server = mockito::Server::new_async().await;
        let mock = server.mock("POST", "/users/test_user")
            .with_status(200)
            .create_async().await;

        let response = fetch_api_register_user_to_server(&server.url(), "test_user").await;
        assert!(response.is_ok());
        mock.assert();
    }

    #[tokio::test]
    async fn test_fetch_api_register_user_to_server_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server.mock("POST", "/users/test_user")
            .with_status(400)
            .with_body(r#"{"error_id":"ERR__USER_ALREADY_EXISTS","error_message":"User already exists"}"#)
            .create_async().await;

        let response = fetch_api_register_user_to_server(&server.url(), "test_user").await;
        assert!(response.is_err());
        assert!(response.err().unwrap().error_id == "ERR__USER_ALREADY_EXISTS");
        mock.assert();
    }

    #[tokio::test]
    async fn test_fetch_api_register_user_to_server_error_fetch() {
        let response = fetch_api_register_user_to_server("http://localhost-non-existent:3012", "test_user").await;
        assert!(response.is_err());
        assert!(response.err().unwrap().error_id == "ERR__CLIENT_FETCH_API");
    }
}
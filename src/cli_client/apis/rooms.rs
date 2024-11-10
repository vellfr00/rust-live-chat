use crate::web_server::handlers::ErrorDetailsResponse;

pub async fn fetch_api_get_room_in_server_by_name(server_endpoint: &str, room_name: &str) -> Result<(), ErrorDetailsResponse> {
    let response = reqwest::get(&format!("{}/rooms/{}", server_endpoint, room_name)).await;
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

pub async fn fetch_api_get_user_in_room_by_name(server_endpoint: &str, room_name: &str, username: &str) -> Result<(), ErrorDetailsResponse> {
    let response = reqwest::get(&format!("{}/rooms/{}/users/{}", server_endpoint, room_name, username)).await;
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

pub async fn fetch_api_create_room_to_server(server_endpoint: &str, room_name: &str, creator_username: &str) -> Result<(), ErrorDetailsResponse> {
    let response = reqwest::Client::new()
        .post(&format!("{}/rooms/{}?creator_username={}", server_endpoint, room_name, creator_username))
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

pub async fn fetch_api_add_user_to_room(server_endpoint: &str, room_name: &str, username: &str) -> Result<(), ErrorDetailsResponse> {
    let response = reqwest::Client::new()
        .post(&format!("{}/rooms/{}/users/{}", server_endpoint, room_name, username))
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
    async fn test_fetch_api_get_room_in_server_by_name() {
        let mut server = mockito::Server::new_async().await;
        let mock = server.mock("GET", "/rooms/test_room")
            .with_status(200)
            .create_async().await;

        let response = fetch_api_get_room_in_server_by_name(&server.url(), "test_room").await;
        assert!(response.is_ok());
        mock.assert();
    }

    #[tokio::test]
    async fn test_fetch_api_get_room_in_server_by_name_room_not_found() {
        let mut server = mockito::Server::new_async().await;
        let mock = server.mock("GET", "/rooms/test_room")
            .with_status(404)
            .with_body(r#"{"error_id":"ERR__ROOM_NOT_FOUND","error_message":"Room not found"}"#)
            .create_async().await;

        let response = fetch_api_get_room_in_server_by_name(&server.url(), "test_room").await;
        assert!(response.is_err());
        assert!(response.err().unwrap().error_id == "ERR__ROOM_NOT_FOUND");
        mock.assert();
    }

    #[tokio::test]
    async fn test_fetch_api_get_room_in_server_by_name_error_fetching() {
        let response = fetch_api_get_room_in_server_by_name("http://localhost-non-existent:3012", "test_room").await;
        assert!(response.is_err());
        assert_eq!(response.err().unwrap().error_id, "ERR__CLIENT_FETCH_API");
    }

    #[tokio::test]
    async fn test_fetch_api_get_user_in_room_by_name() {
        let mut server = mockito::Server::new_async().await;
        let mock = server.mock("GET", "/rooms/test_room/users/test_user")
            .with_status(200)
            .create_async().await;

        let response = fetch_api_get_user_in_room_by_name(&server.url(), "test_room", "test_user").await;
        assert!(response.is_ok());
        mock.assert();
    }

    #[tokio::test]
    async fn test_fetch_api_get_user_in_room_by_name_user_not_found() {
        let mut server = mockito::Server::new_async().await;
        let mock = server.mock("GET", "/rooms/test_room/users/test_user")
            .with_status(404)
            .with_body(r#"{"error_id":"ERR__USER_NOT_FOUND","error_message":"User not found"}"#)
            .create_async().await;

        let response = fetch_api_get_user_in_room_by_name(&server.url(), "test_room", "test_user").await;
        assert!(response.is_err());
        assert!(response.err().unwrap().error_id == "ERR__USER_NOT_FOUND");
        mock.assert();
    }

    #[tokio::test]
    async fn test_fetch_api_get_user_in_room_by_name_room_not_found() {
        let mut server = mockito::Server::new_async().await;
        let mock = server.mock("GET", "/rooms/test_room/users/test_user")
            .with_status(400)
            .with_body(r#"{"error_id":"ERR__ROOM_NOT_FOUND","error_message":"Room not found"}"#)
            .create_async().await;

        let response = fetch_api_get_user_in_room_by_name(&server.url(), "test_room", "test_user").await;
        assert!(response.is_err());
        assert!(response.err().unwrap().error_id == "ERR__ROOM_NOT_FOUND");
        mock.assert();
    }

    #[tokio::test]
    async fn test_fetch_api_get_user_in_room_by_name_error_fetching() {
        let response = fetch_api_get_user_in_room_by_name("http://localhost-non-existent:3012", "test_room", "test_user").await;
        assert!(response.is_err());
        assert_eq!(response.err().unwrap().error_id, "ERR__CLIENT_FETCH_API");
    }

    #[tokio::test]
    async fn test_fetch_api_create_room_to_server() {
        let mut server = mockito::Server::new_async().await;
        let mock = server.mock("POST", "/rooms/test_room?creator_username=test_user")
            .with_status(200)
            .create_async().await;

        let response = fetch_api_create_room_to_server(&server.url(), "test_room", "test_user").await;
        assert!(response.is_ok());
        mock.assert();
    }

    #[tokio::test]
    async fn test_fetch_api_create_room_to_server_room_already_exists() {
        let mut server = mockito::Server::new_async().await;
        let mock = server.mock("POST", "/rooms/test_room?creator_username=test_user")
            .with_status(400)
            .with_body(r#"{"error_id":"ERR__ROOM_ALREADY_EXISTS","error_message":"Room already exists"}"#)
            .create_async().await;

        let response = fetch_api_create_room_to_server(&server.url(), "test_room", "test_user").await;
        assert!(response.is_err());
        assert!(response.err().unwrap().error_id == "ERR__ROOM_ALREADY_EXISTS");
        mock.assert();
    }

    #[tokio::test]
    async fn test_fetch_api_create_room_to_server_user_not_found() {
        let mut server = mockito::Server::new_async().await;
        let mock = server.mock("POST", "/rooms/test_room?creator_username=test_user")
            .with_status(404)
            .with_body(r#"{"error_id":"ERR__USER_NOT_FOUND","error_message":"User not found"}"#)
            .create_async().await;

        let response = fetch_api_create_room_to_server(&server.url(), "test_room", "test_user").await;
        assert!(response.is_err());
        assert!(response.err().unwrap().error_id == "ERR__USER_NOT_FOUND");
        mock.assert();
    }

    #[tokio::test]
    async fn test_fetch_api_create_room_to_server_error_fetching() {
        let response = fetch_api_create_room_to_server("http://localhost-non-existent:3012", "test_room", "test_user").await;
        assert!(response.is_err());
        assert_eq!(response.err().unwrap().error_id, "ERR__CLIENT_FETCH_API");
    }

    #[tokio::test]
    async fn test_fetch_api_add_user_to_room() {
        let mut server = mockito::Server::new_async().await;
        let mock = server.mock("POST", "/rooms/test_room/users/test_user")
            .with_status(200)
            .create_async().await;

        let response = fetch_api_add_user_to_room(&server.url(), "test_room", "test_user").await;
        assert!(response.is_ok());
        mock.assert();
    }

    #[tokio::test]
    async fn test_fetch_api_add_user_to_room_user_already_in_room() {
        let mut server = mockito::Server::new_async().await;
        let mock = server.mock("POST", "/rooms/test_room/users/test_user")
            .with_status(400)
            .with_body(r#"{"error_id":"ERR__USER_ALREADY_IN_ROOM","error_message":"User already in room"}"#)
            .create_async().await;

        let response = fetch_api_add_user_to_room(&server.url(), "test_room", "test_user").await;
        assert!(response.is_err());
        assert!(response.err().unwrap().error_id == "ERR__USER_ALREADY_IN_ROOM");
        mock.assert();
    }

    #[tokio::test]
    async fn test_fetch_api_add_user_to_room_room_not_found() {
        let mut server = mockito::Server::new_async().await;
        let mock = server.mock("POST", "/rooms/test_room/users/test_user")
            .with_status(404)
            .with_body(r#"{"error_id":"ERR__ROOM_NOT_FOUND","error_message":"Room not found"}"#)
            .create_async().await;

        let response = fetch_api_add_user_to_room(&server.url(), "test_room", "test_user").await;
        assert!(response.is_err());
        assert!(response.err().unwrap().error_id == "ERR__ROOM_NOT_FOUND");
        mock.assert();
    }

    #[tokio::test]
    async fn test_fetch_api_add_user_to_room_user_not_found() {
        let mut server = mockito::Server::new_async().await;
        let mock = server.mock("POST", "/rooms/test_room/users/test_user")
            .with_status(404)
            .with_body(r#"{"error_id":"ERR__USER_NOT_FOUND","error_message":"User not found"}"#)
            .create_async().await;

        let response = fetch_api_add_user_to_room(&server.url(), "test_room", "test_user").await;
        assert!(response.is_err());
        assert!(response.err().unwrap().error_id == "ERR__USER_NOT_FOUND");
        mock.assert();
    }

    #[tokio::test]
    async fn test_fetch_api_add_user_to_room_error_fetching() {
        let response = fetch_api_add_user_to_room("http://localhost-non-existent:3012", "test_room", "test_user").await;
        assert!(response.is_err());
        assert_eq!(response.err().unwrap().error_id, "ERR__CLIENT_FETCH_API");
    }
}
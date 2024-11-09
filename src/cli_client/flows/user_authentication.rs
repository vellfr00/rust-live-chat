use crate::{cli_client::api::users::*, web_server::handlers::ErrorDetailsResponse};
use std::io::{self, Write};

fn ask_for_authentication_username() -> String {
    loop {
        print!("Authenticate with your username: ");
        io::stdout().flush().unwrap();
        let mut username = String::new();
        io::stdin().read_line(&mut username).expect("Failed to read line");
        let username = username.trim().to_string();
        if !username.is_empty() {
            return username;
        }
        println!("Username cannot be empty. Please try again.");
    }
}

fn ask_for_registration_username() -> String {
    loop {
        print!("Register with a new username: ");
        io::stdout().flush().unwrap();
        let mut username = String::new();
        io::stdin().read_line(&mut username).expect("Failed to read line");
        let username = username.trim().to_string();
        if !username.is_empty() {
            return username;
        }
        println!("Username cannot be empty. Please try again.");
    }
}

fn ask_if_wants_to_register() -> bool {
    loop {
        print!("User not found in the server. Do you want to register? (y/n): ");
        io::stdout().flush().unwrap();
        let mut register_decision = String::new();
        io::stdin().read_line(&mut register_decision).expect("Failed to read line");
        let register_decision = register_decision.trim().to_string();
        if register_decision == "y" || register_decision == "n" {
            return register_decision == "y";
        }
        println!("Invalid input. Please enter 'y' or 'n'.");
    }
}

fn ask_if_wants_to_authenticate() -> bool {
    loop {
        print!("User already exists in the server. Do you want to authenticate? (y/n): ");
        io::stdout().flush().unwrap();
        let mut authenticate_decision = String::new();
        io::stdin().read_line(&mut authenticate_decision).expect("Failed to read line");
        let authenticate_decision = authenticate_decision.trim().to_string();
        if authenticate_decision == "y" || authenticate_decision == "n" {
            return authenticate_decision == "y";
        }
        println!("Invalid input. Please enter 'y' or 'n'.");
    }
}

async fn authenticate_user(server_endpoint: &str) -> Result<String, ErrorDetailsResponse> {
    let username = ask_for_authentication_username();
    let get_user_response = fetch_api_get_user_in_server_by_username(server_endpoint, &username).await;
    match get_user_response {
        Ok(_) => Ok(username),
        Err(error) => Err(error)
    }
}

async fn register_user(server_endpoint: &str) -> Result<String, ErrorDetailsResponse> {
    let username = ask_for_registration_username();
    let register_user_response = fetch_api_register_user_to_server(server_endpoint, &username).await;
    match register_user_response {
        Ok(_) => Ok(username),
        Err(error) => Err(error)
    }
}

async fn user_authentication_flow(server_endpoint: &str) -> Result<String, ErrorDetailsResponse> {
    let authentication_trial_response = authenticate_user(server_endpoint).await;
    if authentication_trial_response.is_ok() {
        return Ok(authentication_trial_response.unwrap());
    }

    let authentication_trial_error = authentication_trial_response.err().unwrap();
    if authentication_trial_error.error_id == "ERR__USER_NOT_FOUND" {
        let wants_to_register = ask_if_wants_to_register();
        if wants_to_register {
            let registration_trial_response = register_user(server_endpoint).await;
            if registration_trial_response.is_ok() {
                return Ok(registration_trial_response.unwrap());
            }

            let registration_trial_error = registration_trial_response.err().unwrap();
            if registration_trial_error.error_id == "ERR__USER_ALREADY_EXISTS" {
                let wants_to_authenticate = ask_if_wants_to_authenticate();
                if wants_to_authenticate {
                    return authenticate_user(server_endpoint).await;
                }
            }

            return Err(registration_trial_error);
        }
    }

    Err(authentication_trial_error)
}

pub async fn loop_user_authentication_flow(server_endpoint: &str) -> String {
    loop {
        let authentication_result = user_authentication_flow(server_endpoint).await;
        match authentication_result {
            Ok(username) => return username,
            Err(error) => {
                println!("Could not authenticate to server - Please try again. Error was: {}", error.error_message);
            }
        }
    }
}
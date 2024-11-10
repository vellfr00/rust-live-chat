use std::io::{self, Write};

use crate::{cli_client::apis::rooms::*, web_server::handlers::ErrorDetailsResponse};

fn ask_for_room_name_to_enter() -> String {
    loop {
        print!("Enter the name of the room you want to enter: ");
        io::stdout().flush().unwrap();
        let mut room_name = String::new();
        io::stdin().read_line(&mut room_name).expect("Failed to read line");
        let room_name = room_name.trim().to_string();
        if !room_name.is_empty() {
            return room_name;
        }
        println!("Room name cannot be empty. Please try again.");
    }
}

fn ask_if_wants_to_create_room() -> bool {
    loop {
        print!("Room not found in the server. Do you want to create it? (y/n): ");
        io::stdout().flush().unwrap();
        let mut create_decision = String::new();
        io::stdin().read_line(&mut create_decision).expect("Failed to read line");
        let create_decision = create_decision.trim().to_string();
        if create_decision == "y" || create_decision == "n" {
            return create_decision == "y";
        }
        println!("Invalid input. Please enter 'y' or 'n'.");
    }
}

fn ask_for_room_name_to_create() -> String {
    loop {
        print!("Enter the name of the room you want to create: ");
        io::stdout().flush().unwrap();
        let mut room_name = String::new();
        io::stdin().read_line(&mut room_name).expect("Failed to read line");
        let room_name = room_name.trim().to_string();
        if !room_name.is_empty() {
            return room_name;
        }
        println!("Room name cannot be empty. Please try again.");
    }
}

fn ask_if_wants_to_be_added_to_room() -> bool {
    loop {
        print!("Room exists in the server but you are not part of it. Do you want to be added to it? (y/n): ");
        io::stdout().flush().unwrap();
        let mut add_decision = String::new();
        io::stdin().read_line(&mut add_decision).expect("Failed to read line");
        let add_decision = add_decision.trim().to_string();
        if add_decision == "y" || add_decision == "n" {
            return add_decision == "y";
        }
        println!("Invalid input. Please enter 'y' or 'n'.");
    }
}

async fn choose_room(server_endpoint: &str) -> Result<String, ErrorDetailsResponse> {
    let room_name = ask_for_room_name_to_enter();
    let room_exists_response = fetch_api_get_room_in_server_by_name(server_endpoint, &room_name).await;
    match room_exists_response {
        Ok(_) => Ok(room_name),
        Err(error) => Err(error)
    }
}

async fn is_user_in_room(server_endpoint: &str, room_name: &str, username: &str) -> Result<bool, ErrorDetailsResponse> {
    let user_in_room_response = fetch_api_get_user_in_room_by_name(server_endpoint, room_name, username).await;
    match user_in_room_response {
        Ok(_) => Ok(true),
        Err(_) => Ok(false)
    }
}

async fn create_room(server_endpoint: &str, username: &str) -> Result<String, ErrorDetailsResponse> {
    let room_name = ask_for_room_name_to_create();
    let room_create_response = fetch_api_create_room_to_server(server_endpoint, &room_name, username).await;
    match room_create_response {
        Ok(_) => Ok(room_name),
        Err(error) => Err(error)
    }
}

async fn room_choice_flow(server_endpoint: &str, username: &str) -> Result<String, ErrorDetailsResponse> {
    let room_choice_trial = choose_room(server_endpoint).await;
    if room_choice_trial.is_ok() {
        let room_name = room_choice_trial.unwrap();
        let user_in_room_trial = is_user_in_room(server_endpoint, &room_name, username).await;
        if user_in_room_trial.is_ok() {
            let user_in_room = user_in_room_trial.unwrap();
            if user_in_room {
                return Ok(room_name);
            }

            let wants_to_be_added = ask_if_wants_to_be_added_to_room();
            if wants_to_be_added {
                let add_user_to_room_response = fetch_api_add_user_to_room(server_endpoint, &room_name, username).await;
                if add_user_to_room_response.is_ok() {
                    return Ok(room_name);
                }

                return Err(add_user_to_room_response.unwrap_err());
            }

            return Err(ErrorDetailsResponse {
                error_id: "ERR__USER_NOT_ADDED_TO_ROOM".to_string(),
                error_message: "User chose not to be added to the room.".to_string()
            });
        }

        return Err(user_in_room_trial.unwrap_err());
    }

    let wants_to_create_room = ask_if_wants_to_create_room();
    if wants_to_create_room {
        let room_create_trial = create_room(server_endpoint, username).await;
        if room_create_trial.is_ok() {
            return Ok(room_create_trial.unwrap());
        }

        return Err(room_create_trial.unwrap_err());
    }

    return Err(room_choice_trial.unwrap_err());
}

pub async fn loop_room_choice_flow(server_endpoint: &str, username: &str) -> String {
    loop {
        let room_choice_result = room_choice_flow(server_endpoint, username).await;
        match room_choice_result {
            Ok(room_name) => return room_name,
            Err(error) => {
                println!("Could not choose room - Please try again. Error was: {}", error.error_message);
            }
        }
    }
}
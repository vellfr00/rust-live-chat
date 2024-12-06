use std::io::{self, Write};
use chrono::{DateTime, Utc};
use crate::cli_client::apis::rooms::{fetch_api_get_room_messages, fetch_api_post_message_to_room};
use crate::web_server::handlers::ErrorDetailsResponse;
use crate::entities::message::Message;

enum RoomChatChoice {
    ViewMessages,
    SendMessage
}

fn ask_if_wants_to_view_messages_or_send_new_message(username: &str, room_name: &str) -> RoomChatChoice {
    println!("You are in room '{}' as '{}'", room_name, username);
    println!("Do you want to view messages or send a new message?");
    println!("1. View messages");
    println!("2. Send a new message");

    loop {
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
        let mut input_choice = String::new();
        io::stdin().read_line(&mut input_choice).expect("Failed to read line");
        let choice = input_choice.trim().to_string();

        match choice.as_str() {
            "1" => return RoomChatChoice::ViewMessages,
            "2" => return RoomChatChoice::SendMessage,
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn print_messages(messages: &Vec<Message>) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    if messages.is_empty() {
        println!("No messages in this room yet.");
        return;
    }

    for message in messages {
        let datetime: DateTime<Utc> = message.timestamp.into();
        let formatted_timestamp: String = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        println!("[{}] {}: {}", formatted_timestamp, message.author.username, message.content);
    }
}

async fn get_room_messages(server_endpoint: &str, room_name: &str) -> Result<Vec<Message>, ErrorDetailsResponse> {
    let messages = fetch_api_get_room_messages(server_endpoint, room_name).await;
    match messages {
        Ok(messages) => Ok(messages),
        Err(err) => Err(err)
    }
}

async fn send_message(server_endpoint: &str, room_name: &str, username: &str) -> Result<(), ErrorDetailsResponse> {
    loop {
        print!("Enter the message you want to send: ");
        io::stdout().flush().unwrap();
        let mut message_content = String::new();
        io::stdin().read_line(&mut message_content).expect("Failed to read line");
        let message_content = message_content.trim().to_string();
        if !message_content.is_empty() {
            let send_message_response = fetch_api_post_message_to_room(server_endpoint, room_name, username, &message_content).await;
            match send_message_response {
                Ok(_) => return Ok(()),
                Err(err) => return Err(err)
            }
        }
        println!("Message cannot be empty. Please try again.");
    }
}

async fn room_chat_flow(server_endpoint: &str, username: &str, room_name: &str) -> Result<(), ErrorDetailsResponse> {
    loop {
        let choice = ask_if_wants_to_view_messages_or_send_new_message(username, room_name);

        match choice {
            RoomChatChoice::ViewMessages => {
                let messages = get_room_messages(server_endpoint, room_name).await?;
                print_messages(&messages);
            },
            RoomChatChoice::SendMessage => {
                send_message(server_endpoint, room_name, username).await?;
            }
        }
    }
}

pub async fn loop_room_chat_flow(server_endpoint: &str, username: &str, room_name: &str) {
    loop {
        match room_chat_flow(server_endpoint, username, room_name).await {
            Ok(_) => (),
            Err(error) => {
                println!("Could not chat in the room - Please try again. Error was: {}", error.error_message);
            }
        }
    }
}
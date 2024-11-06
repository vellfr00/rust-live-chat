use uuid::Uuid;
use super::{message::Message, user::User};
use std::sync::Arc;

#[derive(Debug)]
pub struct Room {
    pub id: Uuid,
    pub name: String,
    pub users: Vec<Arc<User>>,
    pub messages: Vec<Arc<Message>>,
}

impl Room {
    pub fn new(name: String) -> Self {
        Room {
            id: Uuid::new_v4(),
            name,
            users: Vec::new(),
            messages: Vec::new(),
        }
    }

    pub fn is_user_in_room(&self, user: Arc<User>) -> bool {
        self.users.iter().any(|u| u.to_owned() == user.to_owned())
    }

    pub fn add_user_to_room(&mut self, user: Arc<User>) -> Result<(), &'static str> {
        if self.is_user_in_room(user.clone()) {
            return Err("User is already in the room");
        }

        self.users.push(user);
        Ok(())
    }

    pub fn remove_user_from_room(&mut self, user: Arc<User>) -> Result<(), &'static str> {
        if !self.is_user_in_room(user.clone()) {
            return Err("User is not in the room");
        }

        self.users.retain(|u| u.id != user.id);
        Ok(())
    }

    pub fn post_new_message(&mut self, message: Arc<Message>) -> Result<(), &'static str> {
        let author_user = message.author.clone();
        if(!self.is_user_in_room(author_user)) {
            return Err("User is not in the room");
        }

        self.messages.push(message);
        Ok(())
    }
}
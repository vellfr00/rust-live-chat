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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_room_new() {
        let room = Room::new("test".to_string());
        assert_eq!(room.name, "test");
    }

    #[test]
    fn test_room_is_user_in_room() {
        let user = Arc::new(User::new("test".to_string()));
        let mut room = Room::new("test".to_string());
        room.add_user_to_room(user.clone()).unwrap();
        assert_eq!(room.is_user_in_room(user.clone()), true);
    }

    #[test]
    fn test_room_add_user_to_room() {
        let user = Arc::new(User::new("test".to_string()));
        let mut room = Room::new("test".to_string());
        room.add_user_to_room(user.clone()).unwrap();
        assert_eq!(room.users.len(), 1);
    }

    #[test]
    fn test_room_add_user_to_room_error() {
        let user = Arc::new(User::new("test".to_string()));
        let mut room = Room::new("test".to_string());
        room.add_user_to_room(user.clone()).unwrap();
        let result = room.add_user_to_room(user.clone());
        assert_eq!(result, Err("User is already in the room"));
    }

    #[test]
    fn test_room_remove_user_from_room() {
        let user = Arc::new(User::new("test".to_string()));
        let mut room = Room::new("test".to_string());
        room.add_user_to_room(user.clone()).unwrap();
        room.remove_user_from_room(user.clone()).unwrap();
        assert_eq!(room.users.len(), 0);
    }

    #[test]
    fn test_room_remove_user_from_room_error() {
        let user = Arc::new(User::new("test".to_string()));
        let mut room = Room::new("test".to_string());
        let result = room.remove_user_from_room(user.clone());
        assert_eq!(result, Err("User is not in the room"));
    }

    #[test]
    fn test_room_post_new_message() {
        let user = Arc::new(User::new("test".to_string()));
        let message = Arc::new(Message::new(user.clone(), "test".to_string()));
        let mut room = Room::new("test".to_string());
        room.add_user_to_room(user.clone()).unwrap();
        room.post_new_message(message.clone()).unwrap();
        assert_eq!(room.messages.len(), 1);
    }
}
use uuid::Uuid;
use super::{message::Message, room::Room, user::User};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Server {
    pub id: Uuid,
    pub name: String,
    pub users: Vec<Arc<User>>,
    pub rooms: Vec<Arc<Mutex<Room>>>,
}

impl Server {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            users: Vec::new(),
            rooms: Vec::new(),
        }
    }

    pub fn is_username_already_registered(&self, username: &str) -> bool {
        self.users.iter().any(|user| user.username == username)
    }

    pub fn get_user_by_username(&self, username: &str) -> Option<Arc<User>> {
        self.users.iter().find(|user| user.username == username).map(|user| user.clone())
    }

    pub fn is_room_name_already_registered(&self, room_name: &str) -> bool {
        self.rooms.iter().any(|room| room.lock().unwrap().name == room_name)
    }

    pub fn get_room_by_name(&self, room_name: &str) -> Option<Arc<Mutex<Room>>> {
        self.rooms.iter().find(|room| room.lock().unwrap().name == room_name).map(|room| room.clone())
    }

    pub fn register_user(&mut self, username: &str) -> Result<(), &'static str> {
        if self.is_username_already_registered(username) {
            return Err("Username already registered");
        }

        let user = Arc::new(User::new(username.to_string()));
        self.users.push(user);
        Ok(())
    }

    pub fn create_room(&mut self, room_name: &str, creator_username: &str) -> Result<(), &'static str> {
        if self.is_room_name_already_registered(&room_name) {
            return Err("Room name already registered");
        }

        if !self.is_username_already_registered(&creator_username) {
            return Err("Creator user not registered");
        }

        let room = Arc::new(Mutex::new(Room::new(room_name.to_string())));
        self.rooms.push(room);
        match self.add_user_to_room(&room_name.to_owned(), creator_username) {
            Ok(_) => Ok(()),
            Err(_) => Err("Failed to add creator user to room")
        }
    }

    pub fn add_user_to_room(&mut self, room_name: &str, username: &str) -> Result<(), &'static str> {
        if !self.is_room_name_already_registered(&room_name) {
            return Err("Room name not registered");
        }

        if !self.is_username_already_registered(&username) {
            return Err("Username not registered");
        }

        let room_arc = self.get_room_by_name(&room_name).unwrap();
        let mut room = room_arc.lock().unwrap();
        let user = self.get_user_by_username(&username).unwrap();
        match room.add_user_to_room(user) {
            Ok(_) => Ok(()),
            Err(_) => Err("Failed to add user to room")
        }
    }

    pub fn post_message_to_room(&mut self, room_name: &str, username: &str, message: &str) -> Result<Arc<Message>, &'static str> {
        if !self.is_room_name_already_registered(&room_name) {
            return Err("Room name not registered");
        }

        if !self.is_username_already_registered(&username) {
            return Err("Username not registered");
        }

        let room_arc = self.get_room_by_name(&room_name).unwrap();
        let mut room = room_arc.lock().unwrap();
        let user = self.get_user_by_username(&username).unwrap();
        let message = Arc::new(Message::new(user.clone(), message.to_string()));
        match room.post_new_message(message.clone()) {
            Ok(_) => Ok(message.clone()),
            Err(_) => Err("Failed to post message to room")
        }
    }

    pub fn get_room_messages(&self, room_name: &str) -> Result<Vec<Arc<Message>>, &'static str> {
        if !self.is_room_name_already_registered(&room_name) {
            return Err("Room name not registered");
        }

        let room_arc = self.get_room_by_name(&room_name).unwrap();
        let room = room_arc.lock().unwrap();
        Ok(room.messages.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_new() {
        let server = Server::new("test".to_string());
        assert_eq!(server.name, "test");
    }

    #[test]
    fn test_server_is_username_already_registered() {
        let mut server = Server::new("test".to_string());
        server.register_user("test").unwrap();
        assert_eq!(server.is_username_already_registered("test"), true);
    }

    #[test]
    fn test_server_is_username_already_registered_false() {
        let server = Server::new("test".to_string());
        assert_eq!(server.is_username_already_registered("test"), false);
    }

    #[test]
    fn test_server_get_user_by_username() {
        let mut server = Server::new("test".to_string());
        server.register_user("test").unwrap();
        assert_eq!(server.get_user_by_username("test").unwrap().username, "test");
    }

    #[test]
    fn test_server_get_user_by_username_none() {
        let server = Server::new("test".to_string());
        assert_eq!(server.get_user_by_username("test"), None);
    }

    #[test]
    fn test_server_is_room_name_already_registered() {
        let mut server = Server::new("test".to_string());
        server.register_user("test").unwrap();
        server.create_room("test", "test").unwrap();
        assert_eq!(server.is_room_name_already_registered("test"), true);
    }

    #[test]
    fn test_server_is_room_name_already_registered_false() {
        let server = Server::new("test".to_string());
        assert_eq!(server.is_room_name_already_registered("test"), false);
    }

    #[test]
    fn test_server_get_room_by_name() {
        let mut server = Server::new("test".to_string());
        server.register_user("test").unwrap();
        server.create_room("test", "test").unwrap();
        assert_eq!(server.get_room_by_name("test").unwrap().lock().unwrap().name, "test");
    }

    #[test]
    fn test_server_get_room_by_name_none() {
        let server = Server::new("test".to_string());
        assert!(server.get_room_by_name("test").is_none());
    }

    #[test]
    fn test_server_register_user() {
        let mut server = Server::new("test".to_string());
        server.register_user("test").unwrap();
        assert_eq!(server.users.len(), 1);
    }

    #[test]
    fn test_server_register_user_error() {
        let mut server = Server::new("test".to_string());
        server.register_user("test").unwrap();
        let result = server.register_user("test");
        assert_eq!(result, Err("Username already registered"));
    }

    #[test]
    fn test_server_create_room() {
        let mut server = Server::new("test".to_string());
        server.register_user("test").unwrap();
        server.create_room("test", "test").unwrap();
        assert_eq!(server.rooms.len(), 1);
    }

    #[test]
    fn test_server_create_room_error_room() {
        let mut server = Server::new("test".to_string());
        server.register_user("test").unwrap();
        server.create_room("test", "test").unwrap();
        let result = server.create_room("test", "test");
        assert_eq!(result, Err("Room name already registered"));
    }

    #[test]
    fn test_server_create_room_error_creator() {
        let mut server = Server::new("test".to_string());
        let result = server.create_room("test", "test");
        assert_eq!(result, Err("Creator user not registered"));
    }

    #[test]
    fn test_server_add_user_to_room() {
        let mut server = Server::new("test".to_string());
        server.register_user("test").unwrap();
        server.create_room("test", "test").unwrap();
        server.register_user("test2").unwrap();
        server.add_user_to_room("test", "test2").unwrap();
        assert_eq!(server.get_room_by_name("test").unwrap().lock().unwrap().users.len(), 2);
    }

    #[test]
    fn test_server_add_user_to_room_error_username() {
        let mut server = Server::new("test".to_string());
        server.register_user("test").unwrap();
        server.create_room("test", "test").unwrap();
        let result = server.add_user_to_room("test", "test2");
        assert_eq!(result, Err("Username not registered"));
    }

    #[test]
    fn test_server_add_user_to_room_error_add_user() {
        let mut server = Server::new("test".to_string());
        server.register_user("test").unwrap();
        server.create_room("test", "test").unwrap();
        let result = server.add_user_to_room("test", "test");
        assert_eq!(result, Err("Failed to add user to room"));
    }

    #[test]
    fn test_server_post_message_to_room() {
        let mut server = Server::new("test".to_string());
        server.register_user("test").unwrap();
        server.create_room("test", "test").unwrap();
        let message = server.post_message_to_room("test", "test", "test").unwrap();
        assert!(message.author.username == "test");
        assert!(message.content == "test");
        assert_eq!(server.get_room_messages("test").unwrap().len(), 1);
    }

    #[test]
    fn test_server_post_message_to_room_error_room() {
        let mut server = Server::new("test".to_string());
        server.register_user("test").unwrap();
        server.create_room("test", "test").unwrap();
        let result = server.post_message_to_room("test2", "test", "test");
        assert_eq!(result, Err("Room name not registered"));
    }

    #[test]
    fn test_server_post_message_to_room_error_username() {
        let mut server = Server::new("test".to_string());
        server.register_user("test").unwrap();
        server.create_room("test", "test").unwrap();
        let result = server.post_message_to_room("test", "test2", "test");
        assert_eq!(result, Err("Username not registered"));
    }

    #[test]
    fn test_server_get_room_messages() {
        let mut server = Server::new("test".to_string());
        server.register_user("test").unwrap();
        server.create_room("test", "test").unwrap();
        server.post_message_to_room("test", "test", "test").unwrap();
        assert_eq!(server.get_room_messages("test").unwrap().len(), 1);
    }

    #[test]
    fn test_server_get_room_messages_no_messages() {
        let mut server = Server::new("test".to_string());
        server.register_user("test").unwrap();
        server.create_room("test", "test").unwrap();
        let result = server.get_room_messages("test");
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_server_get_room_messages_error() {
        let mut server = Server::new("test".to_string());
        server.register_user("test").unwrap();
        server.create_room("test", "test").unwrap();
        let result = server.get_room_messages("test2");
        assert_eq!(result, Err("Room name not registered"));
    }
}
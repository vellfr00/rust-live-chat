use std::{sync::Arc, time::SystemTime};
use super::user::User;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub author: Arc<User>,
    pub content: String,
    pub timestamp: SystemTime
}

impl Message {
    pub fn new(author: Arc<User>, content: String) -> Message {
        Message {
            id: Uuid::new_v4(),
            author,
            content,
            timestamp: SystemTime::now()
        }
    }
}

impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_message_new() {
        let user = Arc::new(User::new("test".to_string()));
        let message = Message::new(user.clone(), "test".to_string());
        assert_eq!(message.author, user);
        assert_eq!(message.content, "test");
    }

    #[test]
    fn test_message_eq() {
        let user = Arc::new(User::new("test".to_string()));
        let message1 = Message::new(user.clone(), "test".to_string());
        let message2 = Message::new(user.clone(), "test".to_string());
        assert_ne!(message1, message2);
    }
}
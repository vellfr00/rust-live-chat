use std::{sync::Arc, time::SystemTime};
use super::user::User;
use uuid::Uuid;

#[derive(Debug)]
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
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String
}

impl User {
    pub fn new(username: String) -> User {
        User {
            id: Uuid::new_v4(),
            username
        }
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_new() {
        let user = User::new("user".to_string());

        assert_eq!(user.username, "user");
    }

    #[test]
    fn test_user_eq() {
        let user1 = User::new("user1".to_string());
        let user2 = User::new("user2".to_string());
        let user3 = User::new("user1".to_string());

        assert_ne!(user1, user2);
        assert_ne!(user1, user3);
        assert_eq!(user1, user1);
    }
}
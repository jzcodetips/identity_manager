use crate::domain::User;
use crate::domain::UserStore;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};


#[derive(Clone)]
pub struct InMemUserStore {
    users: Arc<RwLock<HashMap<String, User>>>
}

impl InMemUserStore {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl UserStore for InMemUserStore {
    fn create_user(&self, user: User) -> Result<(), String> {
        let mut users = self.users.write().map_err(|_| "RwLock poisoned".to_string())?;
        users.insert(user.email.clone(), user);
        Ok(())
    }

    fn get_user_by_email(&self, email: &str) -> Result<User, String> {
        let users = self.users.read().map_err(|_| "RwLock poisoned".to_string())?;

        if let Some(user) = users.get(email) {
            Ok(user.clone())
        } else {
            Err("User not found".to_string())
        }
    }
}

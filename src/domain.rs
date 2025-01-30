use uuid::Uuid;
use regex::Regex;
use std::sync::Arc;
use lazy_static::lazy_static;

#[derive(Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
}

pub trait UserStore: Send + Sync + 'static {
    fn create_user(&self, user: User) -> Result<(), String>;
    fn get_user_by_email_and_password(&self, email: &str, password: &str) -> Result<User, String>;
    fn get_user_by_email(&self, email: &str) -> Result<User, String>;
}

#[derive(Clone)]
pub struct Service {
    user_store: Arc<dyn UserStore>
}

impl Service {
    pub fn new<T: UserStore>(user_store: T) -> Self {
        Self {
            user_store: Arc::new(user_store)
        }
    }

    pub fn register(&self, email: &str, password: &str) -> Result<(), String> {
        validate_credentials(email, password)?;
        println!("registering {}", email);
        
        if let Ok(_) = self.user_store.get_user_by_email(email) {
            return Err("Email already registered".to_string());
        }

        let user = User {
            id: Uuid::new_v4().to_string(),
            email: email.to_string(),
            password: password.to_string(),
        };

        self.user_store.create_user(user)
    }

    pub fn login(&self, email: &str, password: &str) -> Result<User, String> {
        validate_credentials(email, password)?;
        self.user_store.get_user_by_email_and_password(email, password)
    }
}

lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
}

fn validate_credentials(email: &str, password: &str) -> Result<(), String> {
    if !EMAIL_REGEX.is_match(email) {
        return Err("Invalid email address.".to_string());
    }

    // Validate password (at least 8 characters, one letter, one digit)
    if password.len() < 8 {
        return Err("Password must be at least 8 characters long.".to_string());
    }

    let has_letter = password.chars().any(|c| c.is_alphabetic());
    let has_digit = password.chars().any(|c| c.is_numeric());

    if !has_letter || !has_digit {
        return Err("Password must contain at least one letter and one digit.".to_string());
    }

    Ok(())
}

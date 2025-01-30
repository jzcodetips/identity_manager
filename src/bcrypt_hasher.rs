use bcrypt::{DEFAULT_COST, hash, verify};
use crate::domain::Hasher;

#[derive(Clone)]
pub struct BcryptHasher;

impl BcryptHasher {
    pub fn new() -> Self {
        Self
    }
}

impl Hasher for BcryptHasher {
    fn hash(&self, password: &str) -> Result<String, String> {
        hash(password, DEFAULT_COST).map_err(|e| e.to_string())
    }

    fn verify(&self, password: &str, hashed_password: &str) -> bool {
        verify(password, hashed_password).unwrap_or(false)
    }
}


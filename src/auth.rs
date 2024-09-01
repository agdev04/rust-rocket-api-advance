use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{PasswordHash, PasswordVerifier, Error}; // Import the Hasher trait
use rand::distributions::Alphanumeric;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use rand::Rng;

use crate::models::User;

#[derive(serde::Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub fn authorize_user(user: &User, credentials: Credentials) -> Result<String, Error> {
    let argon2 = Argon2::default();
    let db_hash = PasswordHash::new(&user.password)?;
    argon2.verify_password(credentials.password.as_bytes(), &db_hash)?;

    let session_id = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(128)
    .map(char::from)
    .collect();

    Ok(session_id)
}

pub fn hash_password(password: String) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    let hash_result = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(hash_result.to_string())
}
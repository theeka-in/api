use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{self, SaltString, rand_core::OsRng},
};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::Rng;

use crate::errors::{ErrorDto, ServiceError};

pub fn generate_token() -> String {
    let mut bytes = [0u8; 48];
    rand::rng().fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

pub fn hash_password(password: &str) -> Result<String, password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, password_hash::Error> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(&password_hash)?;

    let is_the_password_correct = argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    Ok(is_the_password_correct)
}

impl From<password_hash::Error> for ServiceError {
    fn from(e: password_hash::Error) -> Self {
        match e {
            password_hash::Error::Password => ServiceError::Unauthorized(ErrorDto {
                message: "invalid credentials".to_owned(),
            }),
            _ => ServiceError::Internal(ErrorDto {
                message: e.to_string(),
            }),
        }
    }
}
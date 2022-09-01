use pbkdf2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Pbkdf2
};

use std::error::Error;

#[derive(Debug, Clone)]
pub struct Account {
    username: String,
    password: String,
    salt: String
}

pub fn create_account(username: &str, password: &str) -> Result<Account, Box<dyn Error>> {
    let password = password.to_bytes();
    let salt = SaltString::generate(&mut OsRng);

    let password_hash = Pbkdf2.hash_password(password, &salt)?.to_string();

    let parsed_hash = PasswordHash::new(&password_hash)?;
    
    Ok(Account {
        username: username,
        password: password_hash,
        salt: salt,
    })
}

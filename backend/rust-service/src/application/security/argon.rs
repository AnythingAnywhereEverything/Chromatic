use argon2::{
    Argon2,
    password_hash::{
        Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng,
    },
};

pub fn hash(password: &[u8]) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password, &salt)?.to_string();

    Ok(password_hash)
}

pub fn verify(password: &[u8], password_hash: &str) -> Result<bool, Error> {
    let parsed_hash = PasswordHash::new(&password_hash)?;
    match Argon2::default().verify_password(password, &parsed_hash) {
        Ok(_) => Ok(true),
        Err(Error::Password) => Ok(false),
        Err(e) => Err(e),
    }
}

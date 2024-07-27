use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, Params, Algorithm, Version,
};
use rand::Rng;

// use bcrypt::{hash, verify, DEFAULT_COST};

use crate::errors::AppError;

pub fn generate_session_token() -> String {
    let mut rng = rand::thread_rng();
    let token: String = (0..30)
        .map(|_| {
            let idx = rng.gen_range(0..62);
            let chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
            chars[idx] as char
        })
        .collect();
    token
}

// pub fn hash_password(password: &str) -> Result<String, AppError> {
//     let password_bytes = password.as_bytes();
//     let salt = SaltString::generate(&mut OsRng);

//     // Argon2 with default params (Argon2id v19)
//     let argon2 = Argon2::default();

//     // Hash password to PHC string ($argon2id$v=19$...)
//     match argon2.hash_password(password_bytes, &salt) {
//         Ok(hashed_password_bytes) => Ok(hashed_password_bytes.to_string()),
//         Err(_) => Err(AppError::InternalServerError),
//     }
// }

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let password_bytes = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    // カスタムパラメータを設定
    let params = Params::new(1 * 1024, 1, 1, None)
        .map_err(|_| AppError::InternalServerError)?;

    // Argon2 with custom params (Argon2id v19)
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    // Hash password to PHC string ($argon2id$v=19$...)
    match argon2.hash_password(password_bytes, &salt) {
        Ok(hashed_password_bytes) => Ok(hashed_password_bytes.to_string()),
        Err(_) => Err(AppError::InternalServerError),
    }
}

pub fn verify_password(hashed_password: &str, input_password: &str) -> Result<bool, AppError> {
    let input_password_bytes = input_password.as_bytes();
    let parsed_hash = match PasswordHash::new(hashed_password) {
        Ok(hash) => hash,
        Err(_) => return Err(AppError::InternalServerError),
    };
    let argon2 = Argon2::default();
    match argon2.verify_password(input_password_bytes, &parsed_hash) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

// pub fn verify_password(hashed_password: &str, input_password: &str) -> Result<bool, AppError> {
//     let input_password_bytes = input_password.as_bytes();
//     let parsed_hash = match PasswordHash::new(hashed_password) {
//         Ok(hash) => hash,
//         Err(_) => return Err(AppError::InternalServerError),
//     };
//     match Argon2::default().verify_password(input_password_bytes, &parsed_hash) {
//         Ok(_) => Ok(true),
//         Err(_) => Ok(false),
//     }
// }

// pub fn hash_password(password: &str) -> Result<String, AppError> {
//     // ハッシュ化する際のコストをデフォルトに設定（デフォルトは12）
//     match hash(password, DEFAULT_COST) {
//         Ok(hashed_password) => Ok(hashed_password),
//         Err(_) => Err(AppError::InternalServerError),
//     }
// }

// pub fn verify_password(hashed_password: &str, input_password: &str) -> Result<bool, AppError> {
//     match verify(input_password, hashed_password) {
//         Ok(is_valid) => Ok(is_valid),
//         Err(_) => Err(AppError::InternalServerError),
//     }
// }
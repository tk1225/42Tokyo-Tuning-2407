// Argon2ライブラリから必要なモジュールをインポート
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, Params, Algorithm, Version,
};

// ランダムな値を生成するためのRandライブラリをインポート
use rand::Rng;

// bcryptライブラリのインポート (コメントアウトされているため、使用されていない)
// use bcrypt::{hash, verify, DEFAULT_COST};

// 自作のエラー型をインポート
use crate::errors::AppError;

// セッショントークンを生成する関数
pub fn generate_session_token() -> String {
    // スレッドごとの乱数生成器を作成
    let mut rng = rand::thread_rng();
    // 30文字のランダムなトークンを生成
    let token: String = (0..30)
        .map(|_| {
            // 62文字の範囲からランダムにインデックスを選択
            let idx = rng.gen_range(0..62);
            // 使用する文字セット
            let chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
            // 選択したインデックスの文字をトークンに追加
            chars[idx] as char
        })
        .collect();
    token // 生成されたトークンを返す
}

// パスワードをハッシュ化する関数 (コメントアウトされた部分)
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

// パスワードをカスタムパラメータでハッシュ化する関数
pub fn hash_password(password: &str) -> Result<String, AppError> {
    // パスワードをバイト列に変換
    let password_bytes = password.as_bytes();
    // ランダムなソルトを生成
    let salt = SaltString::generate(&mut OsRng);

    // カスタムパラメータを設定
    let params = Params::new(1 * 1024, 1, 1, None)
        .map_err(|_| AppError::InternalServerError)?;

    // カスタムパラメータを使ってArgon2インスタンスを作成
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    // パスワードをハッシュ化してPHC形式の文字列に変換 ($argon2id$v=19$...)
    match argon2.hash_password(password_bytes, &salt) {
        Ok(hashed_password_bytes) => Ok(hashed_password_bytes.to_string()), // 成功時にはハッシュ化されたパスワードを返す
        Err(_) => Err(AppError::InternalServerError), // 失敗時にはエラーを返す
    }
}

// ハッシュ化されたパスワードと入力されたパスワードを検証する関数
pub fn verify_password(hashed_password: &str, input_password: &str) -> Result<bool, AppError> {
    // 入力されたパスワードをバイト列に変換
    let input_password_bytes = input_password.as_bytes();
    // ハッシュ化されたパスワードを解析
    let parsed_hash = match PasswordHash::new(hashed_password) {
        Ok(hash) => hash,
        Err(_) => return Err(AppError::InternalServerError), // 解析に失敗した場合はエラーを返す
    };
    // デフォルトパラメータを使用してArgon2インスタンスを作成
    let argon2 = Argon2::default();
    // パスワードを検証
    match argon2.verify_password(input_password_bytes, &parsed_hash) {
        Ok(_) => Ok(true), // パスワードが一致する場合
        Err(_) => Ok(false), // パスワードが一致しない場合
    }
}

// bcryptを使用したパスワードのハッシュ化と検証 (コメントアウトされた部分)
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

// bcryptを使用したパスワードのハッシュ化
// pub fn hash_password(password: &str) -> Result<String, AppError> {
//     // ハッシュ化する際のコストをデフォルトに設定（デフォルトは12）
//     match hash(password, DEFAULT_COST) {
//         Ok(hashed_password) => Ok(hashed_password),
//         Err(_) => Err(AppError::InternalServerError),
//     }
// }

// bcryptを使用したパスワードの検証
// pub fn verify_password(hashed_password: &str, input_password: &str) -> Result<bool, AppError> {
//     match verify(input_password, hashed_password) {
//         Ok(is_valid) => Ok(is_valid),
//         Err(_) => Err(AppError::InternalServerError),
//     }
// }
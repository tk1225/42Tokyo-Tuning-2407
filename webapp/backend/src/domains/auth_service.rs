use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use std::path::{Path, PathBuf};
use std::process::Command;

use actix_web::web::Bytes;
use log::error;

use crate::errors::AppError;
use crate::models::user::{Dispatcher, Session, User};
use crate::utils::{generate_session_token, hash_password, verify_password};

use super::dto::auth::LoginResponseDto;

pub trait AuthRepository {
    async fn create_user(&self, username: &str, password: &str, role: &str)
        -> Result<(), AppError>;
    async fn find_user_by_id(&self, id: i32) -> Result<Option<User>, AppError>;
    async fn find_user_by_username(&self, username: &str) -> Result<Option<User>, AppError>;
    async fn create_dispatcher(&self, user_id: i32, area_id: i32) -> Result<(), AppError>;
    async fn find_dispatcher_by_id(&self, id: i32) -> Result<Option<Dispatcher>, AppError>;
    async fn find_dispatcher_by_user_id(
        &self,
        user_id: i32,
    ) -> Result<Option<Dispatcher>, AppError>;
    async fn find_profile_image_name_by_user_id(
        &self,
        user_id: i32,
    ) -> Result<Option<String>, AppError>;
    async fn authenticate_user(&self, username: &str, password: &str) -> Result<User, AppError>;
    async fn create_session(&self, user_id: i32, session_token: &str) -> Result<(), AppError>;
    async fn delete_session(&self, session_token: &str) -> Result<(), AppError>;
    async fn find_session_by_session_token(&self, session_token: &str)
        -> Result<Session, AppError>;
}

#[derive(Debug)]
pub struct AuthService<T: AuthRepository + std::fmt::Debug> {
    repository: T,
    image_cache: Arc<Mutex<HashMap<String, Bytes>>>,
}

impl<T: AuthRepository + std::fmt::Debug> AuthService<T> {
    pub fn new(repository: T) -> Self {
        AuthService {
            repository,
            image_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn register_user(
        &self,
        username: &str,
        password: &str,
        role: &str,
        area: Option<i32>,
    ) -> Result<LoginResponseDto, AppError> {
        if role == "dispatcher" && area.is_none() {
            return Err(AppError::BadRequest);
        }

        if (self.repository.find_user_by_username(username).await?).is_some() {
            return Err(AppError::Conflict);
        }

        let hashed_password = hash_password(password).unwrap();

        self.repository
            .create_user(username, &hashed_password, role)
            .await?;

        let session_token = generate_session_token();

        match self.repository.find_user_by_username(username).await? {
            Some(user) => {
                self.repository
                    .create_session(user.id, &session_token)
                    .await?;
                match user.role.as_str() {
                    "dispatcher" => {
                        self.repository
                            .create_dispatcher(user.id, area.unwrap())
                            .await?;
                        let dispatcher = self
                            .repository
                            .find_dispatcher_by_user_id(user.id)
                            .await?
                            .unwrap();
                        Ok(LoginResponseDto {
                            user_id: user.id,
                            username: user.username,
                            session_token,
                            role: user.role,
                            dispatcher_id: Some(dispatcher.id),
                            area_id: Some(dispatcher.area_id),
                        })
                    }
                    _ => Ok(LoginResponseDto {
                        user_id: user.id,
                        username: user.username,
                        session_token,
                        role: user.role,
                        dispatcher_id: None,
                        area_id: None,
                    }),
                }
            }
            None => Err(AppError::InternalServerError),
        }
    }

    pub async fn login_user(
        &self,
        username: &str,
        password: &str,
    ) -> Result<LoginResponseDto, AppError> {
        match self.repository.find_user_by_username(username).await? {
            Some(user) => {
                let is_password_valid = verify_password(&user.password, password).unwrap();
                if !is_password_valid {
                    return Err(AppError::Unauthorized);
                }

                let session_token = generate_session_token();
                let create_session_future = self.repository.create_session(user.id, &session_token);

                let response = match user.role.as_str() {
                    "dispatcher" => {
                        if let Some(dispatcher) =
                            self.repository.find_dispatcher_by_user_id(user.id).await?
                        {
                            LoginResponseDto {
                                user_id: user.id,
                                username: user.username,
                                session_token: session_token.clone(),
                                role: user.role.clone(),
                                dispatcher_id: Some(dispatcher.id),
                                area_id: Some(dispatcher.area_id),
                            }
                        } else {
                            return Err(AppError::InternalServerError);
                        }
                    }
                    _ => LoginResponseDto {
                        user_id: user.id,
                        username: user.username,
                        session_token: session_token.clone(),
                        role: user.role.clone(),
                        dispatcher_id: None,
                        area_id: None,
                    },
                };

                // セッションの作成を待つ
                create_session_future.await?;

                Ok(response)
            }
            None => Err(AppError::Unauthorized),
        }
    }

    pub async fn logout_user(&self, session_token: &str) -> Result<(), AppError> {
        self.repository.delete_session(session_token).await?;
        Ok(())
    }

    pub async fn get_resized_profile_image_byte(&self, user_id: i32) -> Result<Bytes, AppError> {
        let profile_image_name = match self
            .repository
            .find_profile_image_name_by_user_id(user_id)
            .await
        {
            Ok(Some(name)) => name,
            Ok(None) => return Err(AppError::NotFound),
            Err(_) => return Err(AppError::NotFound),
        };

        let cache_key = format!("profile_image_{}", profile_image_name);
        {
            let cache = self.image_cache.lock().unwrap();
            if let Some(cached_image) = cache.get(&cache_key) {
                return Ok(cached_image.clone());
            }
        }

        let path: PathBuf =
            Path::new(&format!("images/user_profile/{}", profile_image_name)).to_path_buf();

        let output = Command::new("magick")
            .arg(&path)
            .arg("-resize")
            .arg("500x500")
            .arg("png:-")
            .output()
            .map_err(|e| {
                error!("画像リサイズのコマンド実行に失敗しました: {:?}", e);
                AppError::InternalServerError
            })?;

        let resized_image = if output.status.success() {
            Bytes::from(output.stdout)
        } else {
            error!(
                "画像リサイズのコマンド実行に失敗しました: {:?}",
                String::from_utf8_lossy(&output.stderr)
            );
            return Err(AppError::InternalServerError);
        };

        {
            let mut cache = self.image_cache.lock().unwrap();
            cache.insert(cache_key, resized_image.clone());
        }

        Ok(resized_image)
    }

    pub async fn validate_session(&self, session_token: &str) -> Result<bool, AppError> {
        let session = self
            .repository
            .find_session_by_session_token(session_token)
            .await?;

        Ok(session.is_valid)
    }
}

use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

// アプリケーションで使用するエラーの定義
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Bad Request")] // リクエストが不正
    BadRequest,
    #[error("Unauthorized")] // 認証が必要
    Unauthorized,
    #[error("Not Found")] // リソースが見つからない
    NotFound,
    #[error("Conflict")] // リソースの競合
    Conflict,
    #[error("Internal Server Error")] // サーバ内部エラー
    InternalServerError,
    #[error(transparent)] // sqlxライブラリからのエラーをラップする
    SqlxError(#[from] sqlx::Error),
}

// エラーレスポンスの構造体
#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

// ResponseErrorトレイトを実装することで、アプリケーションエラーをHTTPレスポンスに変換
impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        // エラーメッセージを文字列に変換
        let error_message = self.to_string();
        // エラーレスポンス構造体を作成
        let error_response = ErrorResponse {
            message: error_message.clone(),
        };

        // エラーメッセージをログに出力
        println!("Error occurred: {}", error_message);

        // エラーの種類に応じたHTTPレスポンスを生成
        match *self {
            AppError::BadRequest => HttpResponse::BadRequest().json(error_response),
            AppError::Unauthorized => HttpResponse::Unauthorized().json(error_response),
            AppError::NotFound => HttpResponse::NotFound().json(error_response),
            AppError::Conflict => HttpResponse::Conflict().json(error_response),
            AppError::InternalServerError => {
                HttpResponse::InternalServerError().json(error_response)
            }
            AppError::SqlxError(_) => HttpResponse::InternalServerError().json(error_response),
        }
    }
}

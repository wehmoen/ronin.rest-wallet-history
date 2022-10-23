use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

pub const DEFAULT_HTTP_USER_AGENT: &str = "Ronin.Rest Wallet History Server 1.0";

#[derive(Serialize, Deserialize)]
pub struct ApiErrorMessage {
    status: i32,
    message: String,
}

impl ApiErrorMessage {
    pub fn create<T>(status: i32, message: T) -> Self
    where
        T: Into<String>,
    {
        ApiErrorMessage {
            status,
            message: message.into(),
        }
    }
}

impl Default for ApiErrorMessage {
    fn default() -> Self {
        ApiErrorMessage {
            status: 9000,
            message: "Not found".into(),
        }
    }
}

pub async fn error_handler() -> HttpResponse {
    HttpResponse::NotFound().json(ApiResponse {
        status: ApiResponseStatus::Error,
        data: None,
        error: Some(ApiErrorMessage::default()),
    })
}

#[derive(Serialize, Deserialize, Default)]
pub enum ApiResponseStatus {
    #[default]
    Ok,
    Error,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ApiResponse {
    pub status: ApiResponseStatus,
    pub data: Option<serde_json::Value>,
    pub error: Option<ApiErrorMessage>,
}

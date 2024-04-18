use actix_web::{http::StatusCode, HttpResponse};
use validator::ValidationErrors;

#[derive(Debug)]
pub struct ApiError {
    message: ApiMessage,
    status: ApiErrorStatus,
}

impl ApiError {
    pub fn new(id: &str, message: &str, status: ApiErrorStatus) -> Self {
        Self { message: ApiMessage::new(id, message), status,}
    }

    pub fn new_from_validation_errors(id: &str, validation_errors: &ValidationErrors) -> Self {
        Self::new(id, &validation_errors.to_string(), ApiErrorStatus::BadRequest)
    }
}

#[derive(Debug)]
pub enum ApiErrorStatus {
    InternalError = 500,
    BadRequest = 400,
    NotFound = 404,
    Conflict = 409,
    NotAcceptable = 406,
    UnAuthorized = 401,
}

#[derive(Debug, serde::Serialize)]
pub struct ApiMessage {
    id: String,
    message: String,
}

impl ApiMessage {
    
    pub fn new(id: &str, message: &str) -> ApiMessage {
        ApiMessage {
            id: id.to_owned(),
            message: message.to_owned(),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ErrorRespnse {
    id: String,
    message: String,
}

impl Into<HttpResponse> for ApiError {
    fn into(self) -> HttpResponse {
        HttpResponse::build(StatusCode::from_u16(self.status as u16).unwrap()).json(self.message)
    }
}

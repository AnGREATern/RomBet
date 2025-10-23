use axum::Json;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct FailureResponse {
    error_message: String,
    error_id: String,
    error_code: String,
    #[serde(skip)]
    status_code: StatusCode,
}

impl FailureResponse {
    pub fn unprocessable_entity(message: impl ToString, error_code: impl ToString) -> Self {
        Self {
            error_message: message.to_string(),
            error_id: Uuid::new_v4().to_string(),
            error_code: error_code.to_string(),
            status_code: StatusCode::UNPROCESSABLE_ENTITY,
        }
    }

    pub fn not_found(message: impl ToString, error_code: impl ToString) -> Self {
        Self {
            error_message: message.to_string(),
            error_id: Uuid::new_v4().to_string(),
            error_code: error_code.to_string(),
            status_code: StatusCode::NOT_FOUND,
        }
    }
}

impl IntoResponse for FailureResponse {
    fn into_response(self) -> Response {
        let status_code = self.status_code;
        let mut resp = Json::from(self).into_response();
        *resp.status_mut() = status_code;

        resp
    }
}

impl From<anyhow::Error> for FailureResponse {
    fn from(value: anyhow::Error) -> Self {
        Self {
            error_message: value.to_string(),
            error_id: Uuid::new_v4().to_string(),
            error_code: "INTERNAL_SERVER_ERROR".into(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

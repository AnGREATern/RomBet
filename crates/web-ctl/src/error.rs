use axum::Json;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct FailureResponse {
    error_message: String,
    error_id: String,
    error_code: String,
}

impl IntoResponse for FailureResponse {
    fn into_response(self) -> Response {
        let status_code = StatusCode::from_str(&self.error_code).unwrap_or(StatusCode::BAD_REQUEST);
        let mut resp = Json::from(self).into_response();
        *resp.status_mut() = status_code;

        resp
    }
}

impl From<anyhow::Error> for FailureResponse {
    fn from(value: anyhow::Error) -> Self {
        FailureResponse {
            error_message: value.to_string(),
            error_id: Uuid::new_v4().to_string(),
            error_code: StatusCode::INTERNAL_SERVER_ERROR.as_str().to_string(),
        }
    }
}

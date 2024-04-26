use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[macro_export]
macro_rules! api_error_message {
    ($message:expr) => {
        crate::core::api_error::builder()
            .with_message($message)
            .build()
            .into_response()
    };
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ApiError {
    #[serde(skip_serializing, skip_deserializing)]
    status: StatusCode,
    message: String,
}

pub struct ApiErrorBuilder {
    api_error: ApiError,
}

pub fn builder() -> ApiErrorBuilder {
    return ApiErrorBuilder {
        api_error: Default::default(),
    };
}

impl ApiErrorBuilder {
    pub fn with_status(self, status: StatusCode) -> Self {
        Self {
            api_error: ApiError {
                status,
                ..self.api_error
            },
        }
    }

    pub fn with_message(self, message: &str) -> Self {
        Self {
            api_error: ApiError {
                message: message.to_owned(),
                ..self.api_error
            },
        }
    }

    pub fn build(self) -> ApiError {
        self.api_error
    }
}

impl Default for ApiError {
    fn default() -> Self {
        Self {
            status: Default::default(),
            message: Default::default(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (self.status, Json(self)).into_response()
    }
}

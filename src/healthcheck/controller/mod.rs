use axum::response::{IntoResponse, Response};

pub struct HealthCheckController;

impl HealthCheckController {
    pub async fn index() -> Response {
        "tres tristes tigres tragan trigo en un trigal".into_response()
    }
}

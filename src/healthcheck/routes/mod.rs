use axum::{routing::get, Router};

use super::controller::HealthCheckController;

pub trait WithHealthCheckRoutes {
    fn with_healthcheck(self) -> Self;
}

impl WithHealthCheckRoutes for Router {
    fn with_healthcheck(self) -> Self {
        let router = Router::new().route("/healthcheck", get(HealthCheckController::index));
        self.merge(router)
    }
}

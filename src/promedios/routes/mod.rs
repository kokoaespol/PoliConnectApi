use axum::{routing::post, Router};

use super::controller::PromediosController;

pub trait WithPromediosRoutes {
    fn with_promedios(self) -> Self;
}

impl WithPromediosRoutes for Router {
    fn with_promedios(self) -> Self {
        self.route("/promedios", post(PromediosController::index))
    }
}

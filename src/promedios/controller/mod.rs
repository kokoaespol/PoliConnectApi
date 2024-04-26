use axum::response::{IntoResponse, Response};
use axum::Json;

use crate::api_error_message;
use crate::core::IntoDomain;
use crate::promedios::domain::use_case::{CalculateGradesUseCase, CalculateMissingUseCase};
use crate::promedios::presentation::{CalculateGradesRequest, CalculateGradesResponse};

pub struct PromediosController;

impl PromediosController {
    pub async fn index(Json(request): Json<CalculateGradesRequest>) -> Response {
        let domain = request.into_domain();

        let Ok(grade) = CalculateGradesUseCase::new().execute(&domain) else {
            return api_error_message!("Invalid grades");
        };

        let Ok(missing) = CalculateMissingUseCase::new().execute(&domain, grade) else {
            return api_error_message!("Invalid grades");
        };

        Json(CalculateGradesResponse {
            grade,
            missing: if missing == 0.0 { None } else { Some(missing) },
            pass: missing == 0.0,
        })
        .into_response()
    }
}

use crate::core::IntoDomain;
use crate::promedios::domain;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CalculateGradesRequest {
    pub primer_parcial: u32,
    pub segundo_parcial: u32,
    pub practico: u32,
    pub mejoramiento: u32,
    pub porcentaje_practico: u32,
}

impl IntoDomain<domain::CalculateGradesRequest> for CalculateGradesRequest {
    fn into_domain(self) -> domain::CalculateGradesRequest {
        domain::CalculateGradesRequest {
            primer_parcial: self.primer_parcial,
            segundo_parcial: self.segundo_parcial,
            practico: self.practico,
            mejoramiento: self.mejoramiento,
            porcentaje_practico: self.porcentaje_practico,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CalculateGradesResponse {
    pub pass: bool,
    pub grade: f32,
    pub missing: Option<f32>,
}

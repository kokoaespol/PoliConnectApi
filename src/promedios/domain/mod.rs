pub mod repository;
pub mod use_case;

/// Payload for calculating grades for a semester.
pub struct CalculateGradesRequest {
    pub primer_parcial: u32,
    pub segundo_parcial: u32,
    pub practico: u32,
    pub mejoramiento: u32,
    pub porcentaje_practico: u32,
}

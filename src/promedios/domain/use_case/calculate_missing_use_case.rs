use crate::promedios::domain::use_case::{Error, Result};
use crate::promedios::domain::CalculateGradesRequest;

pub struct CalculateMissingUseCase;

impl CalculateMissingUseCase {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute(self, request: &CalculateGradesRequest, grade: f32) -> Result {
        if request.porcentaje_practico > 100 {
            return Err(Error::InvalidPorcentajePractico);
        }

        if grade < 60.0 {
            let maxima_teorica = vec![
                request.primer_parcial,
                request.segundo_parcial,
                request.mejoramiento,
            ]
            .into_iter()
            .max()
            .unwrap();

            let porcentaje_practico = request.porcentaje_practico as f32 / 100.0;
            let porcentaje_parcial = (1.0 - porcentaje_practico) / 2.0;

            let teorico = maxima_teorica as f32 * porcentaje_parcial;
            let practico = request.practico as f32 * porcentaje_practico;

            let total = teorico + practico;
            let missing = (60.0 - total) / porcentaje_parcial;

            return Ok(missing);
        }

        Ok(0.0)
    }
}

#[cfg(test)]
mod test {
    use crate::promedios::domain::use_case::CalculateGradesUseCase;

    use super::*;

    #[test]
    fn test_calculate_missing_works() {
        let use_case = CalculateMissingUseCase::new();
        let request = CalculateGradesRequest {
            primer_parcial: 67,
            segundo_parcial: 0,
            mejoramiento: 0,
            porcentaje_practico: 30,
            practico: 0,
        };
        let grade = CalculateGradesUseCase::new().execute(&request).unwrap();

        let result = use_case.execute(&request, grade);

        assert!(result.clone().unwrap() > 104.4 && result.unwrap() < 104.5);
    }
}

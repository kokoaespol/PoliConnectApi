use itertools::Itertools;

use crate::promedios::domain::use_case::{Error, Result};
use crate::promedios::domain::CalculateGradesRequest;

pub struct CalculateGradesUseCase;

impl CalculateGradesUseCase {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute(&self, request: &CalculateGradesRequest) -> Result {
        if request.porcentaje_practico > 100 {
            return Err(Error::InvalidPorcentajePractico);
        }

        let total_teorico = vec![
            request.primer_parcial,
            request.segundo_parcial,
            request.mejoramiento,
        ]
        .into_iter()
        .sorted()
        .skip(1)
        .sum::<u32>();

        let multiplier = request.porcentaje_practico as f32 / 100.0;

        Ok(
            (total_teorico as f32 / 2.0) * (1.0 - multiplier)
                + request.practico as f32 * multiplier,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculating_grades_works_correctly() {
        let use_case = CalculateGradesUseCase::new();
        let payload = CalculateGradesRequest {
            primer_parcial: 69,
            segundo_parcial: 42,
            mejoramiento: 75,
            porcentaje_practico: 30,
            practico: 80,
        };

        let result = use_case.execute(&payload);

        assert!(result.is_ok() && result.clone().unwrap() > 74.3 && result.unwrap() < 74.5);
    }

    #[test]
    fn test_calculating_grades_with_wrong_porcentaje_practico_returns_an_error() {
        let use_case = CalculateGradesUseCase::new();
        let payload = CalculateGradesRequest {
            primer_parcial: 69,
            segundo_parcial: 42,
            mejoramiento: 75,
            porcentaje_practico: 120,
            practico: 80,
        };

        let result = use_case.execute(&payload);

        assert!(result == Err(Error::InvalidPorcentajePractico));
    }
}

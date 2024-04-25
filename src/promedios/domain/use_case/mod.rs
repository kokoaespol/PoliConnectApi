pub mod calculate_grades_use_case;
pub mod calculate_missing_use_case;

pub use calculate_grades_use_case::CalculateGradesUseCase;
pub use calculate_missing_use_case::CalculateMissingUseCase;

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    InvalidPorcentajePractico,
}

pub type Result = std::result::Result<f32, Error>;

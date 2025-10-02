use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize, Error)]
pub enum PayrollError {
    #[error("Invalid date provided")]
    InvalidDate,
    #[error("Employee ID not found")]
    InvalidEmployeeId,
    #[error("Role ID not found")]
    InvalidRoleId,
    #[error("Invalid salary amount")]
    InvalidSalary,
    #[error("Invalid payload: {0}")]
    InvalidPayload(String),
    #[error("Calculation error: {0}")]
    CalculationError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Database connection error")]
    DatabaseConnectionError,
    #[error("Database query error")]
    DatabaseQueryError,
    #[error("Serialization error")]
    SerializationError,
}
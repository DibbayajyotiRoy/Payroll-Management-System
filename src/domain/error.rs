use std::fmt;

#[derive(Debug)]
pub enum PayrollError {
    InvalidDate,
    InvalidEmployeeId,
    InvalidRoleId,
    InvalidSalary,
    InvalidPayload(String),
    CalculationError(String),
    ValidationError(String),
}

impl fmt::Display for PayrollError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PayrollError::InvalidDate => write!(f, "Invalid date provided"),
            PayrollError::InvalidEmployeeId => write!(f, "Employee ID not found"),
            PayrollError::InvalidRoleId => write!(f, "Role ID not found"),
            PayrollError::InvalidSalary => write!(f, "Invalid salary amount"),
            PayrollError::InvalidPayload(msg) => write!(f, "Invalid payload: {}", msg),
            PayrollError::CalculationError(msg) => write!(f, "Calculation error: {}", msg),
            PayrollError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for PayrollError {}

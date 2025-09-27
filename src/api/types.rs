use crate::calculation::types::{AdditionalPayment, Deduction, PayrollCalculationResult};
use crate::domain::models::{AttendanceRecord, CompanyConfiguration, Employee, RoleConfiguration, LeavePolicy};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePayrollRequest {
    pub employees: Vec<Employee>,
    pub company_config: CompanyConfiguration,
    pub role_configs: Vec<RoleConfiguration>,
    pub attendance_records: Vec<AttendanceRecord>,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub additional_payments: Vec<AdditionalPayment>,
    pub deductions: Vec<Deduction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayrollResponse {
    pub success: bool,
    pub results: Vec<PayrollCalculationResult>,
    pub errors: Vec<String>,
    pub processed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoleRequest {
    pub role_name: String,
    pub base_monthly_salary: f64,
    pub currency: String,
    pub overtime_rate_multiplier: f64,
    pub leave_policies: Vec<LeavePolicy>,
    pub working_hours_per_day: f64,
    pub is_active: bool,
}

use crate::calculation::types::{AdditionalPayment, Deduction, PayrollCalculationResult};
use crate::domain::models::{AttendanceRecord, CompanyConfiguration, Employee, RoleConfiguration, LeavePolicy, OvertimePolicy};
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
    pub schema_version: String,
    pub base_salary_minor_units: i64,
    pub currency: String,
    pub overtime_policy: OvertimePolicy,
    pub leave_policies: Vec<LeavePolicy>,
    pub working_hours_per_day: f64,
    pub working_days_per_week: i32,
    pub is_active: bool,
}
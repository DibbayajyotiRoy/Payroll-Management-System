use crate::domain::models::{CompanyConfiguration, RoleConfiguration, AttendanceRecord, LeaveTypeId};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct PayrollCalculationRequest {
    pub employee_id: String,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub company_config: CompanyConfiguration,
    pub role_config: RoleConfiguration,
    pub attendance_records: Vec<AttendanceRecord>,
    pub additional_payments: Vec<AdditionalPayment>,
    pub deductions: Vec<Deduction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdditionalPayment {
    pub id: String,
    pub name: String,
    pub amount: f64,
    pub is_taxable: bool,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deduction {
    pub id: String,
    pub name: String,
    pub amount: f64,
    pub is_pre_tax: bool,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayrollCalculationResult {
    pub employee_id: String,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub calculated_at: DateTime<Utc>,

    // Attendance summary
    pub total_working_days: u32,
    pub present_days: u32,
    pub absent_days: u32,
    pub leave_days: HashMap<LeaveTypeId, u32>,
    pub holiday_days: u32,
    pub weekend_days: u32,

    // Salary calculations
    pub base_salary: f64,
    pub effective_salary: f64, // After attendance adjustments
    pub overtime_hours: f64,
    pub overtime_amount: f64,
    pub additional_payments: f64,
    pub gross_salary: f64,

    // Deductions
    pub attendance_deductions: f64,
    pub leave_deductions: HashMap<LeaveTypeId, f64>,
    pub other_deductions: f64,
    pub total_deductions: f64,

    // Final amounts
    pub net_salary: f64,
    pub currency: String,

    // Detailed breakdown
    pub calculation_steps: Vec<CalculationStep>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalculationStep {
    pub step_type: String,
    pub description: String,
    pub amount: f64,
    pub details: Option<HashMap<String, serde_json::Value>>,
}

use chrono::{NaiveDate, Weekday};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LeaveTypeId(pub String);

impl LeaveTypeId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveType {
    pub id: LeaveTypeId,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeavePolicy {
    pub leave_type_id: LeaveTypeId,
    pub salary_deduction_percent: f64, // 0.0 to 1.0
    pub max_days_per_month: Option<u8>,
    pub max_days_per_year: Option<u16>,
    pub requires_approval: bool,
    pub carry_forward_allowed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleConfiguration {
    #[serde(default = "Default::default")]
    pub role_id: Option<String>,
    pub role_name: String,
    pub base_monthly_salary: f64,
    pub currency: String,
    pub overtime_rate_multiplier: f64,
    pub leave_policies: Vec<LeavePolicy>,
    pub working_hours_per_day: f64,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyConfiguration {
    pub company_id: String,
    pub name: String,
    pub timezone: String,
    pub weekly_off_days: Vec<Weekday>,
    pub special_holidays: Vec<NaiveDate>,
    pub working_days_per_month: u8, // For salary calculation base
    pub default_working_hours: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    pub employee_id: String,
    pub name: String,
    pub email: String,
    pub role_id: String,
    pub hire_date: NaiveDate,
    pub is_active: bool,
    pub custom_salary_override: Option<f64>, // Override role's base salary
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttendanceStatus {
    Present { hours_worked: f64, overtime_hours: f64 },
    Absent { reason: Option<String> },
    OnLeave { leave_type_id: LeaveTypeId, hours_deducted: Option<f64> },
    Holiday,
    WeekOff,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttendanceRecord {
    pub employee_id: String,
    pub date: NaiveDate,
    pub status: AttendanceStatus,
    pub recorded_at: chrono::DateTime<chrono::Utc>,
    pub recorded_by: String, // User ID who recorded this
}

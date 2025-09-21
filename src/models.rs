// src/models.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};
use sqlx::types::Json;
use std::collections::HashMap;

// --- Core Employee & Payroll Models ---

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Employee {
    pub id: Uuid,
    pub employee_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub hire_date: NaiveDate,
    pub department: String,
    pub position: String,
    #[sqlx(json)]
    pub employment_type: EmploymentType,
    #[sqlx(json)]
    pub status: EmployeeStatus,
    pub salary_info: Json<SalaryInfo>,
    pub benefits: Json<Vec<Benefit>>,
    pub tax_info: Json<TaxInfo>,
    pub banking_info: Option<Json<BankingInfo>>, // Should be encrypted in a real application
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum EmploymentType {
    FullTime,
    PartTime,
    Contract,
    Intern,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum EmployeeStatus {
    Active,
    OnLeave,
    Terminated,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SalaryInfo {
    pub base_salary: f64,
    pub currency: String,
    pub pay_frequency: PayFrequency,
    pub overtime_rate: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PayFrequency {
    Weekly,
    BiWeekly,
    Monthly,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Benefit {
    pub benefit_type: String,
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaxInfo {
    pub federal_withholding: f64,
    pub state_withholding: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BankingInfo {
    pub bank_name: String,
    pub routing_number: String,
    pub account_number: String,
    pub crypto_wallet: Option<CryptoWallet>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CryptoWallet {
    pub wallet_address: String,
    pub currency: String,
}

// --- AI-Powered Payroll Analytics ---

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PayrollAnalytics {
    pub id: Uuid,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_gross_pay: f64,
    pub ai_insights: Json<AIInsights>,
    pub fraud_score: f64,
    pub compliance_score: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AIInsights {
    pub cost_predictions: HashMap<String, f64>,
    pub anomalies_detected: Vec<AnomalyReport>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnomalyReport {
    pub employee_id: Uuid,
    pub anomaly_type: String,
    pub description: String,
}


// --- Real-time Payroll Processing ---

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PayrollRun {
    pub id: Uuid,
    #[sqlx(json)]
    pub run_type: RunType,
    #[sqlx(json)]
    pub status: PayrollStatus,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub blockchain_hash: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum RunType {
    Regular,
    Bonus,
    OffCycle,
    EarnedWageAccess,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum PayrollStatus {
    Draft,
    PendingApproval,
    Processing,
    Completed,
    Failed,
}

// --- Employee Self-Service Models ---

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeDashboard {
    pub employee_id: Uuid,
    pub current_pay_period: PayPeriodSummary,
    pub available_earned_wages: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayPeriodSummary {
    pub gross_pay: f64,
    pub deductions: HashMap<String, f64>,
    pub net_pay: f64,
}

// --- ChatBot Integration Models ---

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatBotQuery {
    pub session_id: Uuid,
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatBotResponse {
    pub session_id: Uuid,
    pub response_text: String,
    pub intent: Option<String>,
}
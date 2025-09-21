// src/routes.rs
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use sqlx::{types::Json as SqlxJson, query_as};
use uuid::Uuid;

use crate::{
    db::DbPool,
    models::{Employee, SalaryInfo},
};

// Use a more detailed payload for creating an employee
#[derive(Deserialize)]
pub struct CreateEmployee {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub department: String,
    pub position: String,
    pub salary_info: SalaryInfo,
}

pub fn routes(pool: DbPool) -> Router {
    Router::new()
        // --- Employee Management ---
        .route("/employees", get(list_employees).post(create_employee))
        .route("/employees/:id", get(get_employee))
        
        // --- Placeholder routes for advanced features ---
        .route("/payroll/run", post(run_payroll))
        .route("/analytics/reports", get(get_payroll_analytics))
        .route("/chatbot/query", post(handle_chatbot_query))
        .route("/employee/:id/dashboard", get(get_employee_dashboard))
        .with_state(pool)
}

// --- Employee Route Handlers ---

async fn list_employees(State(pool): State<DbPool>) -> Json<Vec<Employee>> {
    let employees = query_as::<_, Employee>("SELECT * FROM employees ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    Json(employees)
}

async fn get_employee(State(pool): State<DbPool>, Path(id): Path<Uuid>) -> Json<Employee> {
    let employee = query_as::<_, Employee>("SELECT * FROM employees WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .expect("Employee not found");
    
    Json(employee)
}

async fn create_employee(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateEmployee>,
) -> Json<Employee> {
    let employee = sqlx::query_as::<_, Employee>(
        r#"
        INSERT INTO employees (id, employee_id, first_name, last_name, email, hire_date, department, position, employment_type, status, salary_info, benefits, tax_info)
        VALUES ($1, $2, $3, $4, $5, CURRENT_DATE, $6, $7, $8, $9, $10, '[]', '{}')
        RETURNING *
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(format!("EMP-{}", rand::random::<u16>())) // Placeholder for a real ID generator
    .bind(payload.first_name)
    .bind(payload.last_name)
    .bind(payload.email)
    .bind(payload.department)
    .bind(payload.position)
    .bind(SqlxJson(json!("FullTime"))) // Default value
    .bind(SqlxJson(json!("Active")))     // Default value
    .bind(SqlxJson(payload.salary_info))
    .fetch_one(&pool)
    .await
    .expect("Failed to insert employee");

    Json(employee)
}


// --- STUBBED HANDLERS FOR ADVANCED FEATURES ---

// Placeholder for running a payroll cycle
async fn run_payroll() -> Json<serde_json::Value> {
    Json(json!({ "status": "Payroll run initiated successfully", "transaction_id": Uuid::new_v4() }))
}

// Placeholder for fetching AI-powered analytics
async fn get_payroll_analytics() -> Json<serde_json::Value> {
    Json(json!({
        "fraud_score": 0.95,
        "compliance_score": 0.99,
        "insights": [
            "Anomaly detected: 15% increase in overtime for the engineering department.",
            "Prediction: Total payroll cost expected to rise by 2% next quarter."
        ]
    }))
}

// Placeholder for interacting with the AI chatbot
async fn handle_chatbot_query() -> Json<serde_json::Value> {
    Json(json!({ "response": "Your current net pay for this period is $2,543.12." }))
}

// Placeholder for the employee self-service dashboard
async fn get_employee_dashboard(Path(id): Path<Uuid>) -> Json<serde_json::Value> {
    Json(json!({
        "employee_id": id,
        "available_earned_wages": 450.75,
        "ytd_gross_pay": 55123.40
    }))
}
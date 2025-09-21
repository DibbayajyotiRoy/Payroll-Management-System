use axum::{extract::State, Json, routing::{get}, Router};
use serde::Deserialize;
use sqlx::query_as;
use uuid::Uuid;

use crate::{db::DbPool, models::Employee};

#[derive(Deserialize)]
pub struct CreateEmployee {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

pub fn routes(pool: DbPool) -> Router {
    Router::new()
        .route("/employees", get(list_employees).post(create_employee))
        .with_state(pool)
}

async fn list_employees(State(pool): State<DbPool>) -> Json<Vec<Employee>> {
    let employees = query_as::<_, Employee>("SELECT * FROM employees ORDER BY hire_date DESC")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    Json(employees)
}

async fn create_employee(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateEmployee>
) -> Json<Employee> {
    let employee = sqlx::query_as::<_, Employee>(
        "INSERT INTO employees (id, first_name, last_name, email, hire_date)
         VALUES ($1, $2, $3, $4, CURRENT_DATE)
         RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(payload.first_name)
    .bind(payload.last_name)
    .bind(payload.email)
    .fetch_one(&pool)
    .await
    .expect("Failed to insert employee");

    Json(employee)
}

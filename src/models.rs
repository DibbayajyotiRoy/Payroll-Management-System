use serde::{ Serialize};
use uuid::Uuid;
use chrono::NaiveDate;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Employee {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub hire_date: NaiveDate,
}

use actix_web::{web, HttpResponse, Responder};
use crate::db::DbPool;
use crate::domain::models::{Employee, PayrollRun, RoleConfiguration};
use crate::schema::{employees, payroll_runs, role_configurations};
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
struct PayrollSummary {
    employee_id: String,
    employee_name: String,
    base_salary: f64,
    total_deductions: f64,
    net_salary: f64,
}

pub async fn calculate_payroll_handler(
    pool: web::Data<DbPool>,
    employee_id: web::Path<String>,
) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let emp_id = employee_id.into_inner();

    let employee = match employees::table
        .filter(employees::employee_id.eq(&emp_id))
        .first::<Employee>(&mut conn)
    {
        Ok(emp) => emp,
        Err(_) => return HttpResponse::NotFound().body("Employee not found"),
    };

    let role_config = match role_configurations::table
        .filter(role_configurations::role_id.eq(&employee.role_id))
        .first::<RoleConfiguration>(&mut conn)
    {
        Ok(config) => config,
        Err(_) => return HttpResponse::NotFound().body("Role configuration not found"),
    };

    let base_salary = role_config.base_salary_minor_units as f64 / 100.0;
    let deductions = 0.0;
    let net_salary = base_salary - deductions;

    let summary = PayrollSummary {
        employee_id: employee.employee_id,
        employee_name: employee.name,
        base_salary,
        total_deductions: deductions,
        net_salary,
    };

    HttpResponse::Ok().json(summary)
}

pub async fn get_payroll_run_status(
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let run_id = path.into_inner();

    match payroll_runs::table
        .find(run_id)
        .first::<PayrollRun>(&mut conn)
    {
        Ok(run) => HttpResponse::Ok().json(run),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().body("Payroll run not found"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
use crate::api::types::CreateRoleRequest;
use crate::domain::models::RoleConfiguration;
use crate::services::payroll_service::PayrollService;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result};
use std::sync::Arc;
use tokio::sync::Mutex;

// In-memory store for roles (for demonstration purposes)
// In a real application, this would be a database client
pub type AppState = Arc<Mutex<PayrollService>>;

#[post("/roles")]
pub async fn create_role(app_state: web::Data<AppState>, role_config: web::Json<CreateRoleRequest>) -> Result<impl Responder> {
    let mut payroll_service = app_state.lock().await;

    match payroll_service.create_role(role_config.into_inner()).await {
        Ok(role) => Ok(HttpResponse::Created().json(role)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e.to_string())),
    }
}

#[put("/roles/{role_id}")]
pub async fn update_role(app_state: web::Data<AppState>, path: web::Path<String>, role_config: web::Json<RoleConfiguration>) -> Result<impl Responder> {
    let role_id = path.into_inner();
    let mut payroll_service = app_state.lock().await;

    match payroll_service.update_role(role_id, role_config.into_inner()).await {
        Ok(role) => Ok(HttpResponse::Ok().json(role)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e.to_string())),
    }
}

#[get("/roles")]
pub async fn get_all_roles(app_state: web::Data<AppState>) -> Result<impl Responder> {
    let payroll_service = app_state.lock().await;
    let roles = payroll_service.get_all_roles().await;
    Ok(HttpResponse::Ok().json(roles))
}

#[get("/roles/{role_id}")]
pub async fn get_role_by_id(app_state: web::Data<AppState>, path: web::Path<String>) -> Result<impl Responder> {
    let role_id = path.into_inner();
    let payroll_service = app_state.lock().await;
    match payroll_service.get_role_by_id(&role_id).await {
        Some(role) => Ok(HttpResponse::Ok().json(role)),
        None => Ok(HttpResponse::NotFound().body(format!("Role with ID {} not found", role_id))),
    }
}

#[delete("/roles/{role_id}")]
pub async fn delete_role(app_state: web::Data<AppState>, path: web::Path<String>) -> Result<impl Responder> {
    let role_id = path.into_inner();
    let mut payroll_service = app_state.lock().await;
    match payroll_service.delete_role(&role_id).await {
        Ok(true) => Ok(HttpResponse::NoContent().finish()),
        Ok(false) => Ok(HttpResponse::NotFound().body(format!("Role with ID {} not found", role_id))),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e.to_string())),
    }
}

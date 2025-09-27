use crate::api::types::CreatePayrollRequest;
use crate::services::payroll_service::PayrollService;
use actix_web::{post, web, Responder, Result};

#[post("/payroll")]
pub async fn process_payroll_handler(req: web::Json<CreatePayrollRequest>) -> Result<impl Responder> {
    let payroll_service = PayrollService::new();
    let response = payroll_service.process_payroll(req.into_inner());
    Ok(web::Json(response))
}

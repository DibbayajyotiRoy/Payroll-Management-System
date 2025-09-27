use crate::web::{payroll_handler, role_handler};
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(payroll_handler::process_payroll_handler)
       .service(role_handler::create_role)
       .service(role_handler::update_role)
       .service(role_handler::get_all_roles)
       .service(role_handler::get_role_by_id)
       .service(role_handler::delete_role);
}

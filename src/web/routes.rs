use crate::web::{payroll_handler, role_handler, schedule_handler};
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/payroll/{employee_id}")
            .route(web::get().to(payroll_handler::calculate_payroll_handler)),
    )
    .service(
        web::resource("/payroll/runs/{run_id}")
            .route(web::get().to(payroll_handler::get_payroll_run_status)),
    )
    .service(role_handler::create_role)
    .service(role_handler::update_role)
    .service(role_handler::get_all_roles)
    .service(role_handler::get_role_by_id)
    .service(role_handler::delete_role)
    // Add the new schedule routes
    .service(schedule_handler::create_or_update_schedule)
    .service(schedule_handler::toggle_schedule)
    .service(schedule_handler::get_schedule);
}
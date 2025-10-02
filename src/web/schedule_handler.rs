use crate::db::DbPool;
use crate::domain::models::PayrollSchedule;
use actix_web::{get, post, web, HttpResponse, Result};

use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ScheduleRequest {
    company_id: String,
    run_day_of_month: i16,
}

#[post("/schedule")]
pub async fn create_or_update_schedule(
    pool: web::Data<DbPool>,
    req: web::Json<ScheduleRequest>,
) -> Result<HttpResponse> {
    use crate::schema::payroll_schedules::dsl::*;
    let mut conn = pool
        .get()
        .map_err(actix_web::error::ErrorInternalServerError)?;

    diesel::insert_into(payroll_schedules)
        .values((
            company_id.eq(&req.company_id),
            run_day_of_month.eq(req.run_day_of_month),
            is_active.eq(true),
        ))
        .on_conflict(company_id)
        .do_update()
        .set((
            run_day_of_month.eq(req.run_day_of_month),
            is_active.eq(true),
            updated_at.eq(chrono::Utc::now()),
        ))
        .get_result::<PayrollSchedule>(&mut conn)
        .map(|schedule| HttpResponse::Ok().json(schedule))
        .map_err(actix_web::error::ErrorInternalServerError)
}

#[post("/schedule/{company_id}/toggle")]
pub async fn toggle_schedule(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    use crate::schema::payroll_schedules::dsl::*;
    let mut conn = pool
        .get()
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let company_id_path = path.into_inner();

    let schedule = diesel::update(payroll_schedules.filter(company_id.eq(company_id_path)))
        .set(is_active.eq(diesel::dsl::not(is_active)))
        .get_result::<PayrollSchedule>(&mut conn)
        .optional()
        .map_err(actix_web::error::ErrorInternalServerError)?;

    match schedule {
        Some(s) => Ok(HttpResponse::Ok().json(s)),
        None => Ok(HttpResponse::NotFound().body("Schedule not found")),
    }
}

#[get("/schedule/{company_id}")]
pub async fn get_schedule(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    use crate::schema::payroll_schedules::dsl::*;
    let mut conn = pool
        .get()
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let company_id_path = path.into_inner();

    let schedule = payroll_schedules
        .filter(company_id.eq(company_id_path))
        .first::<PayrollSchedule>(&mut conn)
        .optional()
        .map_err(actix_web::error::ErrorInternalServerError)?;

    match schedule {
        Some(s) => Ok(HttpResponse::Ok().json(s)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

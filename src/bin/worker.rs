use basics::api::types::CreatePayrollRequest;
use basics::db::{establish_connection, DbPool};
use basics::domain::models::{
    AttendanceRecord, CompanyConfiguration, Employee, PayrollRun, PayrollRunStatus, // <-- IMPORT THE ENUM
    RoleConfiguration,
};
use basics::schema::{
    company_configurations, employees, payroll_runs, role_configurations, // <-- REMOVED unused attendance_records
};
use basics::services::payroll_service::PayrollService;
use diesel::prelude::*;
use std::time::Duration;

fn find_and_lock_job(conn: &mut PgConnection) -> Result<Option<PayrollRun>, diesel::result::Error> {
    use basics::schema::payroll_runs::dsl::*;

    conn.transaction(|tx| {
        let job = payroll_runs
            .filter(status.eq(PayrollRunStatus::Pending)) // <-- USE ENUM anstatt "pending"
            .order(created_at.asc())
            .limit(1)
            .for_update()
            .skip_locked()
            .first::<PayrollRun>(tx)
            .optional()?;

        if let Some(job) = job {
            let updated_job = diesel::update(&job)
                .set((
                    status.eq(PayrollRunStatus::Running), // <-- USE ENUM anstatt "running"
                    started_at.eq(Some(chrono::Utc::now())),
                ))
                .get_result(tx)?;
            Ok(Some(updated_job))
        } else {
            Ok(None)
        }
    })
}

fn process_payroll_job(pool: &DbPool, job: PayrollRun) {
    println!("[WORKER] Processing payroll run {} for company {}", job.id, job.company_id);
    let mut conn = pool.get().expect("WORKER: Failed to get DB connection for processing.");
    let payroll_service = PayrollService::new(pool.clone());

    let employees: Vec<Employee> = employees::table.filter(employees::company_id.eq(&job.company_id)).load(&mut conn).unwrap();
    let role_configs: Vec<RoleConfiguration> = role_configurations::table.filter(role_configurations::company_id.eq(&job.company_id)).load(&mut conn).unwrap();
    let company_config: CompanyConfiguration = company_configurations::table.find(&job.company_id).first(&mut conn).unwrap();
    // Fetch attendance records specifically for the employees and date range of the job
    let attendance: Vec<AttendanceRecord> = AttendanceRecord::belonging_to(&employees)
        .filter(basics::schema::attendance_records::date.between(job.pay_period_start, job.pay_period_end))
        .load(&mut conn)
        .unwrap();


    let payroll_request = CreatePayrollRequest {
        employees,
        company_config,
        role_configs,
        attendance_records: attendance,
        period_start: job.pay_period_start,
        period_end: job.pay_period_end,
        additional_payments: vec![],
        deductions: vec![],
    };

    let response = payroll_service.process_payroll(payroll_request);

    if response.success {
        diesel::update(payroll_runs::table.find(job.id))
            .set((
                payroll_runs::status.eq(PayrollRunStatus::Completed), // <-- USE ENUM anstatt "completed"
                payroll_runs::completed_at.eq(Some(chrono::Utc::now())),
                payroll_runs::results.eq(serde_json::to_value(&response.results).ok()),
            ))
            .execute(&mut conn)
            .expect("WORKER: DB Error: Failed to mark payroll run as completed.");
        println!("[WORKER] Successfully completed payroll run {}", job.id);
    } else {
        diesel::update(payroll_runs::table.find(job.id))
            .set((
                payroll_runs::status.eq(PayrollRunStatus::Failed), // <-- USE ENUM anstatt "failed"
                payroll_runs::completed_at.eq(Some(chrono::Utc::now())),
                payroll_runs::error_message.eq(Some(response.errors.join(", "))),
            ))
            .execute(&mut conn)
            .expect("WORKER: DB Error: Failed to mark payroll run as failed.");
        eprintln!("[WORKER] Failed payroll run {}: {:?}", job.id, response.errors);
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let pool = establish_connection();
    println!("[WORKER] Worker started. Polling for jobs every 10 seconds...");

    loop {
        let mut conn = pool.get().expect("WORKER: Failed to get DB connection for polling.");
        
        match find_and_lock_job(&mut conn) {
            Ok(Some(job)) => {
                process_payroll_job(&pool, job);
            }
            Ok(None) => {
                tokio::time::sleep(Duration::from_secs(10)).await;
            }
            Err(e) => {
                eprintln!("[WORKER] Database error while polling for jobs: {}. Retrying in 10 seconds.", e);
                tokio::time::sleep(Duration::from_secs(10)).await;
            }
        }
    }
}
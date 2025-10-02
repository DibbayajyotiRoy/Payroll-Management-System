use basics::db::establish_connection;
use basics::domain::models::{PayrollRun, PayrollSchedule};
use basics::schema::{payroll_runs, payroll_schedules};
use chrono::{Datelike, Local, TimeZone};
use diesel::prelude::*;
use tokio_cron_scheduler::{Job, JobScheduler};
use uuid::Uuid;

async fn check_and_schedule_payrolls() {
    println!("[SCHEDULER] Checking for payrolls to run today...");
    let pool = establish_connection();
    let mut conn = pool.get().expect("SCHEDULER: Failed to get DB connection.");
    let today = Local::now().day() as i16;

    let schedules_to_run: Vec<PayrollSchedule> = payroll_schedules::table
        .filter(payroll_schedules::is_active.eq(true))
        .filter(payroll_schedules::run_day_of_month.eq(today))
        .load(&mut conn)
        .expect("DB Error: Could not query payroll schedules.");

    if schedules_to_run.is_empty() {
        println!("[SCHEDULER] No payrolls scheduled for today.");
        return;
    }

    let now = Local::now();
    let first_day_of_month = Local.with_ymd_and_hms(now.year(), now.month(), 1, 0, 0, 0).unwrap().naive_local().date();
    let last_day_of_month = (Local.with_ymd_and_hms(now.year(), now.month() + 1, 1, 0, 0, 0).unwrap() - chrono::Duration::days(1)).naive_local().date();

    for schedule in schedules_to_run {
        println!("[SCHEDULER] Found schedule for company: {}", schedule.company_id);

        let new_run = PayrollRun {
            id: Uuid::new_v4(),
            company_id: schedule.company_id.clone(),
            schedule_id: schedule.id,
            pay_period_start: first_day_of_month,
            pay_period_end: last_day_of_month,
            status: basics::domain::models::PayrollRunStatus::Pending,
            results: None,
            error_message: None,
            created_at: chrono::Utc::now(),
            started_at: None,
            completed_at: None,
        };

        match diesel::insert_into(payroll_runs::table)
            .values(&new_run)
            .execute(&mut conn)
        {
            Ok(_) => println!("[SCHEDULER] Created payroll job {} for company {}", new_run.id, new_run.company_id),
            Err(e) => eprintln!("[SCHEDULER] Error creating payroll job: {}", e),
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let sched = JobScheduler::new().await.unwrap();

    let job = Job::new_async("0 0 1 * * *", |_uuid, _l| {
        Box::pin(check_and_schedule_payrolls())
    }).unwrap();

    sched.add(job).await.unwrap();
    println!("[SCHEDULER] Scheduler is running. Will check for jobs daily at 1 a.m.");
    sched.start().await.unwrap();
    
    loop {
        tokio::time::sleep(core::time::Duration::from_secs(60 * 60)).await;
    }
}
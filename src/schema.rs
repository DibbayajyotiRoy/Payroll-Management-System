// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId , diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "attendance_status_enum"))]
    pub struct AttendanceStatusEnum;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "payroll_run_status"))]
    pub struct PayrollRunStatus;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AttendanceStatusEnum;

    attendance_records (id) {
        id -> Int4,
        employee_id -> Varchar,
        date -> Date,
        status -> AttendanceStatusEnum,
        hours_worked -> Nullable<Float8>,
        overtime_hours -> Nullable<Float8>,
        reason -> Nullable<Text>,
        leave_type_id -> Nullable<Varchar>,
        hours_deducted -> Nullable<Float8>,
        recorded_at -> Timestamptz,
        recorded_by -> Varchar,
    }
}

diesel::table! {
    company_configurations (company_id) {
        company_id -> Varchar,
        name -> Varchar,
        timezone -> Varchar,
        weekly_off_days -> Array<Nullable<Text>>,
        special_holidays -> Array<Nullable<Date>>,
        working_days_per_month -> Int2,
        default_working_hours -> Float8,
    }
}

diesel::table! {
    employees (employee_id) {
        employee_id -> Varchar,
        company_id -> Varchar,
        name -> Varchar,
        email -> Varchar,
        role_id -> Varchar,
        hire_date -> Date,
        is_active -> Bool,
        custom_salary_override -> Nullable<Float8>,
    }
}

diesel::table! {
    leave_policies (id) {
        id -> Int4,
        role_id -> Varchar,
        leave_type_id -> Varchar,
        salary_deduction_percent -> Float8,
        max_days_per_month -> Nullable<Int2>,
        max_days_per_year -> Nullable<Int2>,
        requires_approval -> Bool,
        carry_forward_allowed -> Bool,
    }
}

diesel::table! {
    leave_types (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PayrollRunStatus;

    payroll_runs (id) {
        id -> Uuid,
        company_id -> Varchar,
        schedule_id -> Int4,
        pay_period_start -> Date,
        pay_period_end -> Date,
        status -> PayrollRunStatus,
        results -> Nullable<Jsonb>,
        error_message -> Nullable<Text>,
        created_at -> Timestamptz,
        started_at -> Nullable<Timestamptz>,
        completed_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    payroll_schedules (id) {
        id -> Int4,
        company_id -> Varchar,
        run_day_of_month -> Int2,
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    role_configurations (role_id) {
        role_id -> Varchar,
        company_id -> Varchar,
        role_name -> Varchar,
        schema_version -> Varchar,
        base_salary_minor_units -> Int8,
        currency -> Varchar,
        overtime_policy -> Jsonb,
        leave_policies -> Jsonb,
        working_hours_per_day -> Float8,
        working_days_per_week -> Int4,
        is_active -> Bool,
    }
}

diesel::joinable!(attendance_records -> leave_types (leave_type_id));
diesel::joinable!(leave_policies -> leave_types (leave_type_id));
diesel::joinable!(leave_policies -> role_configurations (role_id));
diesel::joinable!(payroll_runs -> company_configurations (company_id));
diesel::joinable!(payroll_runs -> payroll_schedules (schedule_id));
diesel::joinable!(payroll_schedules -> company_configurations (company_id));

diesel::allow_tables_to_appear_in_same_query!(
    attendance_records,
    company_configurations,
    employees,
    leave_policies,
    leave_types,
    payroll_runs,
    payroll_schedules,
    role_configurations,
);

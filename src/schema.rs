// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "attendance_status_enum"))]
    pub struct AttendanceStatusEnum;
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
        name -> Varchar,
        email -> Varchar,
        role_id -> Varchar,
        hire_date -> Date,
        is_active -> Bool,
        custom_salary_override -> Nullable<Float8>,
    }
}

diesel::table! {
    role_configurations (role_id) {
        role_id -> Varchar,
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

diesel::joinable!(attendance_records -> employees (employee_id));

diesel::allow_tables_to_appear_in_same_query!(
    attendance_records,
    company_configurations,
    employees,
    role_configurations,
);
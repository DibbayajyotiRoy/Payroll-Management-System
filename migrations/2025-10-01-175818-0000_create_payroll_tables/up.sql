CREATE TABLE leave_types (
    id VARCHAR PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT
);

CREATE TABLE role_configurations (
    role_id VARCHAR PRIMARY KEY,
    role_name VARCHAR NOT NULL,
    base_monthly_salary DOUBLE PRECISION NOT NULL,
    currency VARCHAR NOT NULL,
    overtime_rate_multiplier DOUBLE PRECISION NOT NULL,
    working_hours_per_day DOUBLE PRECISION NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE leave_policies (
    id SERIAL PRIMARY KEY,
    role_id VARCHAR NOT NULL REFERENCES role_configurations(role_id),
    leave_type_id VARCHAR NOT NULL REFERENCES leave_types(id),
    salary_deduction_percent DOUBLE PRECISION NOT NULL,
    max_days_per_month SMALLINT,
    max_days_per_year SMALLINT,
    requires_approval BOOLEAN NOT NULL DEFAULT TRUE,
    carry_forward_allowed BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE company_configurations (
    company_id VARCHAR PRIMARY KEY,
    name VARCHAR NOT NULL,
    timezone VARCHAR NOT NULL,
    weekly_off_days TEXT[] NOT NULL,
    special_holidays DATE[] NOT NULL,
    working_days_per_month SMALLINT NOT NULL,
    default_working_hours DOUBLE PRECISION NOT NULL
);

CREATE TYPE attendance_status_enum AS ENUM ('present', 'absent', 'on_leave', 'holiday', 'week_off');

CREATE TABLE attendance_records (
    id SERIAL PRIMARY KEY,
    employee_id VARCHAR NOT NULL REFERENCES employees(employee_id),
    date DATE NOT NULL,
    status attendance_status_enum NOT NULL,
    hours_worked DOUBLE PRECISION,
    overtime_hours DOUBLE PRECISION,
    reason TEXT,
    leave_type_id VARCHAR REFERENCES leave_types(id),
    hours_deducted DOUBLE PRECISION,
    recorded_at TIMESTAMPTZ NOT NULL,
    recorded_by VARCHAR NOT NULL,
    UNIQUE(employee_id, date)
);
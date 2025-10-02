CREATE TABLE employees (
    employee_id VARCHAR PRIMARY KEY,
    company_id VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    role_id VARCHAR NOT NULL,
    hire_date DATE NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    custom_salary_override DOUBLE PRECISION
);
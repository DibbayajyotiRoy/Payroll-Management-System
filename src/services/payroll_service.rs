use crate::api::types::{CreatePayrollRequest, CreateRoleRequest, PayrollResponse};
use crate::calculation::engine::PayrollCalculationEngine;
use crate::calculation::types::{PayrollCalculationRequest, PayrollCalculationResult};
use crate::domain::error::PayrollError;
use crate::domain::models::{AttendanceRecord, RoleConfiguration, LeavePolicy, JsonbValue};
use crate::db::DbPool;
use crate::schema::role_configurations;
use diesel::prelude::*;
use chrono::Utc;
use uuid::Uuid;
use serde_json::to_value;

pub struct BatchPayrollProcessor {
    engine: PayrollCalculationEngine,
}

impl BatchPayrollProcessor {
    pub fn new() -> Self {
        Self {
            engine: PayrollCalculationEngine::new(),
        }
    }

    pub fn process_batch(&self, requests: Vec<PayrollCalculationRequest>) -> Vec<Result<PayrollCalculationResult, PayrollError>> {
        requests.into_iter()
            .map(|request| self.engine.calculate(request))
            .collect()
    }

    pub async fn process_batch_async(&self, requests: Vec<PayrollCalculationRequest>) -> Vec<Result<PayrollCalculationResult, PayrollError>> {
        self.process_batch(requests)
    }
}

pub struct PayrollService {
    processor: BatchPayrollProcessor,
    pool: DbPool,
}

impl PayrollService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            processor: BatchPayrollProcessor::new(),
            pool,
        }
    }

    pub async fn create_role(&mut self, role_config: CreateRoleRequest) -> Result<RoleConfiguration, PayrollError> {
        let mut conn = self.pool.get().map_err(|_| PayrollError::DatabaseConnectionError)?;
        let role_id = Uuid::new_v4().to_string();
        let new_role = RoleConfiguration {
            role_id: role_id.clone(),
            role_name: role_config.role_name,
            schema_version: role_config.schema_version,
            base_salary_minor_units: role_config.base_salary_minor_units,
            currency: role_config.currency,
            overtime_policy: JsonbValue(to_value(role_config.overtime_policy).map_err(|_| PayrollError::SerializationError)?),
            leave_policies: JsonbValue(to_value(role_config.leave_policies).map_err(|_| PayrollError::SerializationError)?),
            working_hours_per_day: role_config.working_hours_per_day,
            working_days_per_week: role_config.working_days_per_week,
            is_active: role_config.is_active,
        };

        diesel::insert_into(role_configurations::table)
            .values(&new_role)
            .execute(&mut conn)
            .map_err(|_| PayrollError::DatabaseQueryError)?; // TODO: Map to a more specific error

        Ok(new_role)
    }

    pub async fn update_role(&mut self, role_id: String, role_config: RoleConfiguration) -> Result<RoleConfiguration, PayrollError> {
        let mut conn = self.pool.get().map_err(|_| PayrollError::DatabaseConnectionError)?;
        
        diesel::update(role_configurations::table.find(role_id))
            .set(&role_config)
            .execute(&mut conn)
            .map_err(|_| PayrollError::DatabaseQueryError)?;

        Ok(role_config)
    }

    pub async fn get_all_roles(&self) -> Result<Vec<RoleConfiguration>, PayrollError> {
        let mut conn = self.pool.get().map_err(|_| PayrollError::DatabaseConnectionError)?;
        role_configurations::table
            .load::<RoleConfiguration>(&mut conn)
            .map_err(|_| PayrollError::DatabaseQueryError)
    }

    pub async fn get_role_by_id(&self, role_id: &str) -> Result<Option<RoleConfiguration>, PayrollError> {
        let mut conn = self.pool.get().map_err(|_| PayrollError::DatabaseConnectionError)?;
        role_configurations::table
            .find(role_id)
            .first::<RoleConfiguration>(&mut conn)
            .optional()
            .map_err(|_| PayrollError::DatabaseQueryError)
    }

    pub async fn delete_role(&mut self, role_id: &str) -> Result<bool, PayrollError> {
        let mut conn = self.pool.get().map_err(|_| PayrollError::DatabaseConnectionError)?;
        let num_deleted = diesel::delete(role_configurations::table.find(role_id))
            .execute(&mut conn)
            .map_err(|_| PayrollError::DatabaseQueryError)?;
        Ok(num_deleted > 0)
    }

    pub fn process_payroll(&self, request: CreatePayrollRequest) -> PayrollResponse {
        let mut results = Vec::new();
        let mut errors = Vec::new();

        // This part of the logic might need to be updated to fetch role configs from the DB
        // if they are not passed in the request.
        for employee in &request.employees {
            if let Some(role_config) = request.role_configs.iter().find(|config| config.role_id == employee.role_id) {
                let employee_attendance: Vec<AttendanceRecord> = request.attendance_records
                    .iter()
                    .filter(|record| record.employee_id == employee.employee_id)
                    .cloned()
                    .collect();

                let employee_payments = request.additional_payments.clone();
                let employee_deductions = request.deductions.clone();

                let calc_request = PayrollCalculationRequest {
                    employee_id: employee.employee_id.clone(),
                    period_start: request.period_start,
                    period_end: request.period_end,
                    company_config: request.company_config.clone(),
                    role_config: role_config.clone(),
                    attendance_records: employee_attendance,
                    additional_payments: employee_payments,
                    deductions: employee_deductions,
                };

                match self.processor.engine.calculate(calc_request) {
                    Ok(result) => results.push(result),
                    Err(error) => errors.push(format!("Employee {}: {}", employee.employee_id, error)),
                }
            } else {
                errors.push(format!("Role configuration not found for employee {}: {}",
                                   employee.employee_id, employee.role_id));
            }
        }

        PayrollResponse {
            success: errors.is_empty(),
            results,
            errors,
            processed_at: Utc::now(),
        }
    }

    pub fn validate_configuration(&self, config: &RoleConfiguration) -> Vec<String> {
        let mut validation_errors = Vec::new();

        if config.base_salary_minor_units <= 0 {
            validation_errors.push("Base monthly salary must be greater than 0".to_string());
        }

        if config.working_hours_per_day <= 0.0 || config.working_hours_per_day > 24.0 {
            validation_errors.push("Working hours per day must be between 0 and 24".to_string());
        }

        if config.working_days_per_week <= 0 || config.working_days_per_week > 7 {
            validation_errors.push("Working days per week must be between 1 and 7".to_string());
        }

        let overtime_policy: crate::domain::models::OvertimePolicy = serde_json::from_value(config.overtime_policy.0.clone()).unwrap_or_default();
        if overtime_policy.weekday_multiplier <= 0.0 || overtime_policy.weekend_multiplier <= 0.0 || overtime_policy.holiday_multiplier <= 0.0 {
            validation_errors.push("Overtime multipliers must be greater than 0".to_string());
        }

        let leave_policies: Vec<LeavePolicy> = serde_json::from_value(config.leave_policies.0.clone()).unwrap_or_default();
        for policy in &leave_policies {
            if policy.deduction.deduction_type == "percent" && (policy.deduction.value < 0.0 || policy.deduction.value > 100.0) {
                validation_errors.push(format!("Invalid deduction percentage for leave type {}: must be between 0.0 and 100.0", policy.leave_type_name));
            }
        }

        validation_errors
    }
}
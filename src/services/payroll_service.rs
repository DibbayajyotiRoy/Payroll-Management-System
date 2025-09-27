use crate::api::types::{CreatePayrollRequest, CreateRoleRequest, PayrollResponse};
use crate::calculation::engine::PayrollCalculationEngine;
use crate::calculation::types::{PayrollCalculationRequest, PayrollCalculationResult};
use crate::domain::error::PayrollError;
use crate::domain::models::{AttendanceRecord, RoleConfiguration};
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

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
    // In-memory store for roles (for demonstration purposes)
    // In a real application, this would be a database client
    roles: Arc<Mutex<HashMap<String, RoleConfiguration>>>,
}

impl PayrollService {
    pub fn new() -> Self {
        Self {
            processor: BatchPayrollProcessor::new(),
            roles: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn create_role(&mut self, role_config: CreateRoleRequest) -> Result<RoleConfiguration, PayrollError> {
        let mut roles = self.roles.lock().await;
        let role_id = Uuid::new_v4().to_string();
        let new_role = RoleConfiguration {
            role_id: Some(role_id.clone()),
            role_name: role_config.role_name,
            base_monthly_salary: role_config.base_monthly_salary,
            currency: role_config.currency,
            overtime_rate_multiplier: role_config.overtime_rate_multiplier,
            leave_policies: role_config.leave_policies,
            working_hours_per_day: role_config.working_hours_per_day,
            is_active: role_config.is_active,
        };

        roles.insert(role_id, new_role.clone());
        Ok(new_role)
    }

    pub async fn update_role(&mut self, role_id: String, role_config: RoleConfiguration) -> Result<RoleConfiguration, PayrollError> {
        let mut roles = self.roles.lock().await;
        if !roles.contains_key(&role_id) {
            return Err(PayrollError::ValidationError(format!("Role with ID {} not found", role_id)));
        }
        roles.insert(role_id, role_config.clone());
        Ok(role_config)
    }

    pub async fn get_all_roles(&self) -> Vec<RoleConfiguration> {
        let roles = self.roles.lock().await;
        roles.values().cloned().collect()
    }

    pub async fn get_role_by_id(&self, role_id: &str) -> Option<RoleConfiguration> {
        let roles = self.roles.lock().await;
        roles.get(role_id).cloned()
    }

    pub async fn delete_role(&mut self, role_id: &str) -> Result<bool, PayrollError> {
        let mut roles = self.roles.lock().await;
        match roles.remove(role_id) {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    pub fn process_payroll(&self, request: CreatePayrollRequest) -> PayrollResponse {
        let mut results = Vec::new();
        let mut errors = Vec::new();

        for employee in &request.employees {
            if let Some(role_config) = request.role_configs.iter().find(|config| config.role_id.as_ref() == Some(&employee.role_id)) {
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

        if config.base_monthly_salary <= 0.0 {
            validation_errors.push("Base monthly salary must be greater than 0".to_string());
        }

        if config.overtime_rate_multiplier <= 0.0 {
            validation_errors.push("Overtime rate multiplier must be greater than 0".to_string());
        }

        if config.working_hours_per_day <= 0.0 || config.working_hours_per_day > 24.0 {
            validation_errors.push("Working hours per day must be between 0 and 24".to_string());
        }

        for policy in &config.leave_policies {
            if policy.salary_deduction_percent < 0.0 || policy.salary_deduction_percent > 1.0 {
                validation_errors.push(format!("Invalid salary deduction percentage for leave type {}: must be between 0.0 and 1.0", policy.leave_type_id.0));
            }
        }

        validation_errors
    }
}

use crate::domain::error::PayrollError;
use crate::domain::models::*;
use crate::calculation::types::*;
use chrono::{Datelike, Duration, Utc};
use std::collections::HashMap;

pub struct PayrollCalculationEngine;

impl PayrollCalculationEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn calculate(&self, request: PayrollCalculationRequest) -> Result<PayrollCalculationResult, PayrollError> {
        self.validate_request(&request)?;

        let mut result = PayrollCalculationResult {
            employee_id: request.employee_id.clone(),
            period_start: request.period_start,
            period_end: request.period_end,
            calculated_at: Utc::now(),
            total_working_days: 0,
            present_days: 0,
            absent_days: 0,
            leave_days: HashMap::new(),
            holiday_days: 0,
            weekend_days: 0,
            base_salary: 0.0,
            effective_salary: 0.0,
            overtime_hours: 0.0,
            overtime_amount: 0.0,
            additional_payments: 0.0,
            gross_salary: 0.0,
            attendance_deductions: 0.0,
            leave_deductions: HashMap::new(),
            other_deductions: 0.0,
            total_deductions: 0.0,
            net_salary: 0.0,
            currency: request.role_config.currency.clone(),
            calculation_steps: Vec::new(),
        };

        self.calculate_attendance_summary(&request, &mut result)?;
        self.calculate_base_salary(&request, &mut result)?;
        self.calculate_overtime(&request, &mut result)?;
        self.calculate_attendance_deductions(&request, &mut result)?;
        self.calculate_leave_deductions(&request, &mut result)?;
        self.apply_additional_payments(&request, &mut result)?;
        self.apply_other_deductions(&request, &mut result)?;
        self.calculate_final_amounts(&mut result)?;

        Ok(result)
    }

    fn validate_request(&self, request: &PayrollCalculationRequest) -> Result<(), PayrollError> {
        if request.period_start > request.period_end {
            return Err(PayrollError::InvalidDate);
        }

        if request.role_config.base_monthly_salary < 0.0 {
            return Err(PayrollError::InvalidSalary);
        }

        if request.employee_id.is_empty() {
            return Err(PayrollError::InvalidEmployeeId);
        }

        for policy in &request.role_config.leave_policies {
            if policy.salary_deduction_percent < 0.0 || policy.salary_deduction_percent > 1.0 {
                return Err(PayrollError::ValidationError(
                    format!("Invalid deduction percentage for leave type: {:?}", policy.leave_type_id)
                ));
            }
        }

        Ok(())
    }

    fn calculate_attendance_summary(&self, request: &PayrollCalculationRequest, result: &mut PayrollCalculationResult) -> Result<(), PayrollError> {
        let mut current_date = request.period_start;
        let mut step_details = HashMap::new();

        while current_date <= request.period_end {
            if self.is_weekend(&request.company_config, current_date) {
                result.weekend_days += 1;
            } else if self.is_holiday(&request.company_config, current_date) {
                result.holiday_days += 1;
            } else {
                result.total_working_days += 1;

                if let Some(record) = request.attendance_records.iter().find(|r| r.date == current_date) {
                    match &record.status {
                        AttendanceStatus::Present { hours_worked: _, overtime_hours } => {
                            result.present_days += 1;
                            result.overtime_hours += overtime_hours;
                        },
                        AttendanceStatus::Absent { .. } => {
                            result.absent_days += 1;
                        },
                        AttendanceStatus::OnLeave { leave_type_id, .. } => {
                            *result.leave_days.entry(leave_type_id.clone()).or_insert(0) += 1;
                        },
                        AttendanceStatus::Holiday => {
                            result.holiday_days += 1;
                        },
                        AttendanceStatus::WeekOff => {
                            result.weekend_days += 1;
                        },
                    }
                } else {
                    result.absent_days += 1;
                }
            }
            current_date += Duration::days(1);
        }

        step_details.insert("total_working_days".to_string(), serde_json::Value::Number(result.total_working_days.into()));
        step_details.insert("present_days".to_string(), serde_json::Value::Number(result.present_days.into()));
        step_details.insert("absent_days".to_string(), serde_json::Value::Number(result.absent_days.into()));

        result.calculation_steps.push(CalculationStep {
            step_type: "attendance_summary".to_string(),
            description: format!("Calculated attendance: {} working days, {} present, {} absent",
                               result.total_working_days, result.present_days, result.absent_days),
            amount: 0.0,
            details: Some(step_details),
        });

        Ok(())
    }

    fn calculate_base_salary(&self, request: &PayrollCalculationRequest, result: &mut PayrollCalculationResult) -> Result<(), PayrollError> {
        result.base_salary = request.role_config.base_monthly_salary;

        if result.total_working_days > 0 {
            let expected_working_days = request.company_config.working_days_per_month as u32;
            if result.total_working_days != expected_working_days {
                let pro_rated_salary = (result.base_salary / expected_working_days as f64) * result.total_working_days as f64;
                result.base_salary = pro_rated_salary;
            }
        }

        result.calculation_steps.push(CalculationStep {
            step_type: "base_salary".to_string(),
            description: format!("Base salary calculated: {} {}", result.base_salary, result.currency),
            amount: result.base_salary,
            details: None,
        });

        Ok(())
    }

    fn calculate_overtime(&self, request: &PayrollCalculationRequest, result: &mut PayrollCalculationResult) -> Result<(), PayrollError> {
        if result.overtime_hours > 0.0 {
            let hourly_rate = result.base_salary / (result.total_working_days as f64 * request.role_config.working_hours_per_day);
            result.overtime_amount = result.overtime_hours * hourly_rate * request.role_config.overtime_rate_multiplier;

            result.calculation_steps.push(CalculationStep {
                step_type: "overtime".to_string(),
                description: format!("Overtime: {:.1} hours × {:.2} rate × {:.1}x = {} {}",
                                   result.overtime_hours, hourly_rate,
                                   request.role_config.overtime_rate_multiplier,
                                   result.overtime_amount, result.currency),
                amount: result.overtime_amount,
                details: None,
            });
        }

        Ok(())
    }

    fn calculate_attendance_deductions(&self, _request: &PayrollCalculationRequest, result: &mut PayrollCalculationResult) -> Result<(), PayrollError> {
        if result.absent_days > 0 {
            let daily_rate = result.base_salary / result.total_working_days as f64;
            result.attendance_deductions = result.absent_days as f64 * daily_rate;

            result.calculation_steps.push(CalculationStep {
                step_type: "attendance_deduction".to_string(),
                description: format!("Absent days deduction: {} days × {:.2} = {} {}",
                                   result.absent_days, daily_rate,
                                   result.attendance_deductions, result.currency),
                amount: result.attendance_deductions,
                details: None,
            });
        }

        Ok(())
    }

    fn calculate_leave_deductions(&self, request: &PayrollCalculationRequest, result: &mut PayrollCalculationResult) -> Result<(), PayrollError> {
        let daily_rate = if result.total_working_days > 0 {
            result.base_salary / result.total_working_days as f64
        } else {
            0.0
        };

        for (leave_type_id, days) in &result.leave_days {
            if let Some(policy) = request.role_config.leave_policies.iter()
                .find(|p| &p.leave_type_id == leave_type_id) {

                let deduction = (*days as f64) * daily_rate * policy.salary_deduction_percent;
                result.leave_deductions.insert(leave_type_id.clone(), deduction);

                if deduction > 0.0 {
                    result.calculation_steps.push(CalculationStep {
                        step_type: "leave_deduction".to_string(),
                        description: format!("Leave deduction ({}): {} days × {:.2} × {:.0}% = {} {}",
                                           leave_type_id.0, days, daily_rate,
                                           policy.salary_deduction_percent * 100.0,
                                           deduction, result.currency),
                        amount: deduction,
                        details: None,
                    });
                }
            } else {
                let deduction = (*days as f64) * daily_rate;
                result.leave_deductions.insert(leave_type_id.clone(), deduction);

                result.calculation_steps.push(CalculationStep {
                    step_type: "leave_deduction".to_string(),
                    description: format!("Unknown leave type ({}): {} days × {:.2} = {} {}",
                                       leave_type_id.0, days, daily_rate,
                                       deduction, result.currency),
                    amount: deduction,
                    details: None,
                });
            }
        }

        Ok(())
    }

    fn apply_additional_payments(&self, request: &PayrollCalculationRequest, result: &mut PayrollCalculationResult) -> Result<(), PayrollError> {
        result.additional_payments = request.additional_payments.iter()
            .map(|payment| payment.amount)
            .sum();

        if result.additional_payments > 0.0 {
            for payment in &request.additional_payments {
                result.calculation_steps.push(CalculationStep {
                    step_type: "additional_payment".to_string(),
                    description: format!("Additional payment: {} = {} {}",
                                       payment.name, payment.amount, result.currency),
                    amount: payment.amount,
                    details: None,
                });
            }
        }

        Ok(())
    }

    fn apply_other_deductions(&self, request: &PayrollCalculationRequest, result: &mut PayrollCalculationResult) -> Result<(), PayrollError> {
        result.other_deductions = request.deductions.iter()
            .map(|deduction| deduction.amount)
            .sum();

        if result.other_deductions > 0.0 {
            for deduction in &request.deductions {
                result.calculation_steps.push(CalculationStep {
                    step_type: "other_deduction".to_string(),
                    description: format!("Deduction: {} = {} {}",
                                       deduction.name, deduction.amount, result.currency),
                    amount: deduction.amount,
                    details: None,
                });
            }
        }

        Ok(())
    }

    fn calculate_final_amounts(&self, result: &mut PayrollCalculationResult) -> Result<(), PayrollError> {
        result.effective_salary = result.base_salary - result.attendance_deductions;

        let total_leave_deductions: f64 = result.leave_deductions.values().sum();
        result.effective_salary -= total_leave_deductions;

        result.gross_salary = result.effective_salary + result.overtime_amount + result.additional_payments;

        result.total_deductions = result.attendance_deductions + total_leave_deductions + result.other_deductions;

        result.net_salary = result.gross_salary - result.other_deductions;

        result.calculation_steps.push(CalculationStep {
            step_type: "final_calculation".to_string(),
            description: format!("Final: Gross {} - Deductions {} = Net {} {}",
                               result.gross_salary, result.total_deductions,
                               result.net_salary, result.currency),
            amount: result.net_salary,
            details: None,
        });

        Ok(())
    }

    fn is_weekend(&self, config: &CompanyConfiguration, date: chrono::NaiveDate) -> bool {
        if config.weekly_off_days.is_empty() {
            return date.weekday() == chrono::Weekday::Sun;
        }
        config.weekly_off_days.contains(&date.weekday())
    }

    fn is_holiday(&self, config: &CompanyConfiguration, date: chrono::NaiveDate) -> bool {
        config.special_holidays.contains(&date)
    }
}

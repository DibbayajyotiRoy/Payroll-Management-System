use basics::calculation::engine::PayrollCalculationEngine;
use basics::calculation::types::{AdditionalPayment, Deduction, PayrollCalculationRequest};
use basics::config::builder::PayrollConfigurationBuilder;
use basics::domain::models::{AttendanceRecord, AttendanceStatus, LeaveTypeId, LeavePolicy};
use basics::services::payroll_service::BatchPayrollProcessor;
use chrono::{NaiveDate, Utc};

#[test]
fn test_dynamic_payroll_calculation() {
    let engine = PayrollCalculationEngine::new();

    let request = PayrollCalculationRequest {
        employee_id: "EMP001".to_string(),
        period_start: NaiveDate::from_ymd_opt(2024, 3, 1).unwrap(),
        period_end: NaiveDate::from_ymd_opt(2024, 3, 31).unwrap(),
        company_config: PayrollConfigurationBuilder::create_sample_company_config(),
        role_config: PayrollConfigurationBuilder::create_sample_role_config(),
        attendance_records: vec![
            AttendanceRecord {
                employee_id: "EMP001".to_string(),
                date: NaiveDate::from_ymd_opt(2024, 3, 1).unwrap(),
                status: AttendanceStatus::Present {
                    hours_worked: 8.0,
                    overtime_hours: 2.0,
                },
                recorded_at: Utc::now(),
                recorded_by: "SYSTEM".to_string(),
            },
            AttendanceRecord {
                employee_id: "EMP001".to_string(),
                date: NaiveDate::from_ymd_opt(2024, 3, 4).unwrap(),
                status: AttendanceStatus::Absent {
                    reason: Some("Personal".to_string()),
                },
                recorded_at: Utc::now(),
                recorded_by: "SYSTEM".to_string(),
            },
            AttendanceRecord {
                employee_id: "EMP001".to_string(),
                date: NaiveDate::from_ymd_opt(2024, 3, 5).unwrap(),
                status: AttendanceStatus::OnLeave {
                    leave_type_id: LeaveTypeId::new("SICK"),
                    hours_deducted: None,
                },
                recorded_at: Utc::now(),
                recorded_by: "SYSTEM".to_string(),
            },
        ],
        additional_payments: vec![
            AdditionalPayment {
                id: "BONUS001".to_string(),
                name: "Performance Bonus".to_string(),
                amount: 500.0,
                is_taxable: true,
                description: Some("Q1 Performance".to_string()),
            }
        ],
        deductions: vec![
            Deduction {
                id: "TAX001".to_string(),
                name: "Income Tax".to_string(),
                amount: 800.0,
                is_pre_tax: false,
                description: Some("Federal Tax".to_string()),
            }
        ],
    };

    let result = engine.calculate(request).unwrap();

    println!("=== DYNAMIC PAYROLL CALCULATION RESULT ===");
    println!("Employee ID: {}", result.employee_id);
    println!("Period: {} to {}", result.period_start, result.period_end);
    println!("Working Days: {}", result.total_working_days);
    println!("Present Days: {}", result.present_days);
    println!("Absent Days: {}", result.absent_days);
    println!("Leave Days: {:?}", result.leave_days);
    println!("Base Salary: {} {}", result.base_salary, result.currency);
    println!("Overtime Amount: {} {}", result.overtime_amount, result.currency);
    println!("Gross Salary: {} {}", result.gross_salary, result.currency);
    println!("Total Deductions: {} {}", result.total_deductions, result.currency);
    println!("Net Salary: {} {}", result.net_salary, result.currency);

    println!("\n=== CALCULATION BREAKDOWN ===");
    for step in &result.calculation_steps {
        println!("[{}] {}", step.step_type.to_uppercase(), step.description);
    }

    assert!(result.net_salary > 0.0);
    assert!(result.overtime_amount > 0.0);
    assert!(result.total_deductions > 0.0);
}

#[test]
fn test_custom_leave_policies() {
    let engine = PayrollCalculationEngine::new();

    let mut role_config = PayrollConfigurationBuilder::create_sample_role_config();
    role_config.leave_policies = vec![
        LeavePolicy {
            leave_type_id: LeaveTypeId::new("MATERNITY"),
            salary_deduction_percent: 0.0,
            max_days_per_month: None,
            max_days_per_year: Some(90),
            requires_approval: true,
            carry_forward_allowed: false,
        },
        LeavePolicy {
            leave_type_id: LeaveTypeId::new("STUDY"),
            salary_deduction_percent: 0.5,
            max_days_per_month: Some(5),
            max_days_per_year: Some(30),
            requires_approval: true,
            carry_forward_allowed: false,
        },
    ];

    let request = PayrollCalculationRequest {
        employee_id: "EMP002".to_string(),
        period_start: NaiveDate::from_ymd_opt(2024, 3, 1).unwrap(),
        period_end: NaiveDate::from_ymd_opt(2024, 3, 31).unwrap(),
        company_config: PayrollConfigurationBuilder::create_sample_company_config(),
        role_config,
        attendance_records: vec![
            AttendanceRecord {
                employee_id: "EMP002".to_string(),
                date: NaiveDate::from_ymd_opt(2024, 3, 5).unwrap(),
                status: AttendanceStatus::OnLeave {
                    leave_type_id: LeaveTypeId::new("STUDY"),
                    hours_deducted: None,
                },
                recorded_at: Utc::now(),
                recorded_by: "SYSTEM".to_string(),
            },
            AttendanceRecord {
                employee_id: "EMP002".to_string(),
                date: NaiveDate::from_ymd_opt(2024, 3, 6).unwrap(),
                status: AttendanceStatus::OnLeave {
                    leave_type_id: LeaveTypeId::new("MATERNITY"),
                    hours_deducted: None,
                },
                recorded_at: Utc::now(),
                recorded_by: "SYSTEM".to_string(),
            },
        ],
        additional_payments: vec![],
        deductions: vec![],
    };

    let result = engine.calculate(request).unwrap();

    println!("=== CUSTOM LEAVE POLICIES TEST ===");
    println!("Leave Days: {:?}", result.leave_days);
    println!("Leave Deductions: {:?}", result.leave_deductions);

    assert!(result.leave_deductions.contains_key(&LeaveTypeId::new("STUDY")));
    assert!(!result.leave_deductions.contains_key(&LeaveTypeId::new("MATERNITY"))
            || result.leave_deductions[&LeaveTypeId::new("MATERNITY")] == 0.0);
}

#[test]
fn test_batch_processing() {
    let processor = BatchPayrollProcessor::new();

    let requests = vec![
        PayrollCalculationRequest {
            employee_id: "EMP001".to_string(),
            period_start: NaiveDate::from_ymd_opt(2024, 3, 1).unwrap(),
            period_end: NaiveDate::from_ymd_opt(2024, 3, 31).unwrap(),
            company_config: PayrollConfigurationBuilder::create_sample_company_config(),
            role_config: PayrollConfigurationBuilder::create_sample_role_config(),
            attendance_records: vec![],
            additional_payments: vec![],
            deductions: vec![],
        },
        PayrollCalculationRequest {
            employee_id: "EMP002".to_string(),
            period_start: NaiveDate::from_ymd_opt(2024, 3, 1).unwrap(),
            period_end: NaiveDate::from_ymd_opt(2024, 3, 31).unwrap(),
            company_config: PayrollConfigurationBuilder::create_sample_company_config(),
            role_config: PayrollConfigurationBuilder::create_sample_role_config(),
            attendance_records: vec![],
            additional_payments: vec![],
            deductions: vec![],
        },
    ];

    let results = processor.process_batch(requests);

    assert_eq!(results.len(), 2);
    assert!(results[0].is_ok());
    assert!(results[1].is_ok());

    println!("Batch processing completed successfully for {} employees", results.len());
}

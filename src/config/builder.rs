use crate::domain::models::{CompanyConfiguration, LeavePolicy, LeaveTypeId, RoleConfiguration};
use chrono::{NaiveDate, Weekday};

pub struct PayrollConfigurationBuilder;

impl PayrollConfigurationBuilder {
    pub fn create_sample_company_config() -> CompanyConfiguration {
        CompanyConfiguration {
            company_id: "COMP001".to_string(),
            name: "Tech Solutions Inc".to_string(),
            timezone: "UTC".to_string(),
            weekly_off_days: vec![Weekday::Sat, Weekday::Sun],
            special_holidays: vec![
                NaiveDate::from_ymd_opt(2024, 12, 25).unwrap(), // Christmas
                NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),   // New Year
            ],
            working_days_per_month: 22,
            default_working_hours: 8.0,
        }
    }

    pub fn create_sample_role_config() -> RoleConfiguration {
        RoleConfiguration {
            role_id: "ROLE_SENIOR_DEV".to_string(),
            role_name: "Senior Developer".to_string(),
            base_monthly_salary: 8000.0,
            currency: "USD".to_string(),
            overtime_rate_multiplier: 1.5,
            leave_policies: vec![
                LeavePolicy {
                    leave_type_id: LeaveTypeId::new("SICK"),
                    salary_deduction_percent: 0.0, // Paid
                    max_days_per_month: Some(3),
                    max_days_per_year: Some(12),
                    requires_approval: false,
                    carry_forward_allowed: true,
                },
                LeavePolicy {
                    leave_type_id: LeaveTypeId::new("CASUAL"),
                    salary_deduction_percent: 0.0, // Paid
                    max_days_per_month: Some(2),
                    max_days_per_year: Some(15),
                    requires_approval: true,
                    carry_forward_allowed: false,
                },
                LeavePolicy {
                    leave_type_id: LeaveTypeId::new("ANNUAL"),
                    salary_deduction_percent: 0.0, // Paid
                    max_days_per_month: None,
                    max_days_per_year: Some(25),
                    requires_approval: true,
                    carry_forward_allowed: true,
                },
                LeavePolicy {
                    leave_type_id: LeaveTypeId::new("UNPAID"),
                    salary_deduction_percent: 1.0, // Full deduction
                    max_days_per_month: None,
                    max_days_per_year: None,
                    requires_approval: true,
                    carry_forward_allowed: false,
                },
            ],
            working_hours_per_day: 8.0,
            is_active: true,
        }
    }
}

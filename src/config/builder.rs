use crate::domain::models::{CompanyConfiguration, RoleConfiguration, OvertimePolicy, LeavePolicy, Deduction, JsonbValue};
use chrono::NaiveDate;
use serde_json::json;

pub struct PayrollConfigurationBuilder;

impl PayrollConfigurationBuilder {
    pub fn create_sample_company_config() -> CompanyConfiguration {
        CompanyConfiguration {
            company_id: "COMP001".to_string(),
            name: "Tech Solutions Inc".to_string(),
            timezone: "UTC".to_string(),
            weekly_off_days: vec![Some("Sat".to_string()), Some("Sun".to_string())],
            special_holidays: vec![
                Some(NaiveDate::from_ymd_opt(2024, 12, 25).unwrap()), // Christmas
                Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),   // New Year
            ],
            working_days_per_month: 22,
            default_working_hours: 8.0,
        }
    }

    pub fn create_sample_role_config() -> RoleConfiguration {
        let overtime_policy = OvertimePolicy {
            weekday_multiplier: 1.5,
            weekend_multiplier: 2.0,
            holiday_multiplier: 3.0,
        };

        let leave_policies = vec![
            LeavePolicy {
                leave_type_id: "1".to_string(),
                leave_type_name: "sick_leave".to_string(),
                deduction: Deduction {
                    deduction_type: "percent".to_string(),
                    value: 50.0,
                },
                max_days_per_month: 5,
                max_days_per_year: 60,
                requires_approval: true,
                carry_forward_allowed: true,
            },
            LeavePolicy {
                leave_type_id: "2".to_string(),
                leave_type_name: "paid_leave".to_string(),
                deduction: Deduction {
                    deduction_type: "none".to_string(),
                    value: 0.0,
                },
                max_days_per_month: 2,
                max_days_per_year: 24,
                requires_approval: true,
                carry_forward_allowed: false,
            },
        ];

        RoleConfiguration {
            role_id: "1".to_string(),
            role_name: "Software Engineer".to_string(),
            schema_version: "1.0".to_string(),
            base_salary_minor_units: 7500000,
            currency: "INR".to_string(),
            overtime_policy: JsonbValue(json!(overtime_policy)),
            leave_policies: JsonbValue(json!(leave_policies)),
            working_hours_per_day: 8.0,
            working_days_per_week: 5,
            is_active: true,
        }
    }
}
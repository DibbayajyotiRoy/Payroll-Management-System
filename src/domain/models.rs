use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::*;
use diesel::sql_types::{Text, Jsonb};
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, ToSql, Output};
use diesel::backend::Backend;
use serde_json::Value as Json;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, FromSqlRow, AsExpression)]
#[diesel(sql_type = Text)]
pub struct LeaveTypeId(pub String);

impl ToSql<Text, diesel::pg::Pg> for LeaveTypeId {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::pg::Pg>) -> serialize::Result {
        ToSql::<Text, diesel::pg::Pg>::to_sql(&self.0, out)
    }
}

impl<DB: Backend> FromSql<Text, DB> for LeaveTypeId
where
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let s = <String as FromSql<Text, DB>>::from_sql(bytes)?;
        Ok(LeaveTypeId(s))
    }
}


impl LeaveTypeId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

// Newtype wrapper for serde_json::Value to implement Diesel traits
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, FromSqlRow, AsExpression)]
#[diesel(sql_type = Jsonb)]
pub struct JsonbValue(pub Json);

impl<DB: Backend> FromSql<Jsonb, DB> for JsonbValue
where
    Json: FromSql<Jsonb, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let value = <Json as FromSql<Jsonb, DB>>::from_sql(bytes)?;
        Ok(JsonbValue(value))
    }
}

impl<DB: Backend> ToSql<Jsonb, DB> for JsonbValue
where
    Json: ToSql<Jsonb, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        <Json as ToSql<Jsonb, DB>>::to_sql(&self.0, out)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OvertimePolicy {
    pub weekday_multiplier: f64,
    pub weekend_multiplier: f64,
    pub holiday_multiplier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Deduction {
    #[serde(rename = "type")]
    pub deduction_type: String,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LeavePolicy {
    pub leave_type_id: String,
    pub leave_type_name: String,
    pub deduction: Deduction,
    pub max_days_per_month: i32,
    pub max_days_per_year: i32,
    pub requires_approval: bool,
    pub carry_forward_allowed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset, Identifiable)]
#[diesel(table_name = role_configurations)]
#[diesel(primary_key(role_id))]
pub struct RoleConfiguration {
    pub role_id: String,
    pub role_name: String,
    pub schema_version: String,
    pub base_salary_minor_units: i64,
    pub currency: String,
    pub overtime_policy: JsonbValue,
    pub leave_policies: JsonbValue,
    pub working_hours_per_day: f64,
    pub working_days_per_week: i32,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset, Identifiable)]
#[diesel(table_name = company_configurations)]
#[diesel(primary_key(company_id))]
pub struct CompanyConfiguration {
    pub company_id: String,
    pub name: String,
    pub timezone: String,
    pub weekly_off_days: Vec<Option<String>>,
    pub special_holidays: Vec<Option<NaiveDate>>,
    pub working_days_per_month: i16,
    pub default_working_hours: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset, Identifiable)]
#[diesel(table_name = employees)]
#[diesel(primary_key(employee_id))]
pub struct Employee {
    pub employee_id: String,
    pub name: String,
    pub email: String,
    pub role_id: String,
    pub hire_date: NaiveDate,
    pub is_active: bool,
    pub custom_salary_override: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, DbEnum, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::AttendanceStatusEnum"]
pub enum AttendanceStatus {
    Present,
    Absent,
    OnLeave,
    Holiday,
    WeekOff,
}


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset, Associations, Identifiable)]
#[diesel(belongs_to(Employee, foreign_key = employee_id))]
#[diesel(table_name = attendance_records)]
#[diesel(primary_key(id))]
pub struct AttendanceRecord {
    pub id: i32,
    pub employee_id: String,
    pub date: NaiveDate,
    pub status: AttendanceStatus,
    pub hours_worked: Option<f64>,
    pub overtime_hours: Option<f64>,
    pub reason: Option<String>,
    pub leave_type_id: Option<LeaveTypeId>,
    pub hours_deducted: Option<f64>,
    pub recorded_at: chrono::DateTime<chrono::Utc>,
    pub recorded_by: String,
}
# RoleConfiguration


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**company_id** | **string** | The ID of the company this role belongs to. | [default to undefined]
**role_name** | **string** | The human-readable name of the role. | [default to undefined]
**schema_version** | **string** | The version of this configuration schema. | [optional] [default to undefined]
**base_salary_minor_units** | **number** | Base monthly salary in the smallest currency unit (e.g., cents) to avoid floating-point issues. | [default to undefined]
**currency** | **string** | The ISO 4217 currency code. | [default to undefined]
**overtime_policy** | [**OvertimePolicy**](OvertimePolicy.md) |  | [default to undefined]
**leave_policies** | [**Array&lt;LeavePolicy&gt;**](LeavePolicy.md) | A list of policies for different types of leave. | [default to undefined]
**working_hours_per_day** | **number** |  | [default to undefined]
**working_days_per_week** | **number** |  | [default to undefined]
**is_active** | **boolean** |  | [default to undefined]
**role_id** | **string** | Unique identifier for the role configuration, generated on creation. | [optional] [default to undefined]

## Example

```typescript
import { RoleConfiguration } from './api';

const instance: RoleConfiguration = {
    company_id,
    role_name,
    schema_version,
    base_salary_minor_units,
    currency,
    overtime_policy,
    leave_policies,
    working_hours_per_day,
    working_days_per_week,
    is_active,
    role_id,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

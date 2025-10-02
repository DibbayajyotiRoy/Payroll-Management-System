# PayrollCalculationResult

Detailed results of a payroll calculation for a single employee.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**employee_id** | **string** |  | [optional] [default to undefined]
**period_start** | **string** |  | [optional] [default to undefined]
**period_end** | **string** |  | [optional] [default to undefined]
**net_salary** | **number** | The final take-home pay for the employee. | [optional] [default to undefined]
**gross_salary** | **number** | Total earnings before any deductions. | [optional] [default to undefined]
**total_deductions** | **number** | The sum of all deductions. | [optional] [default to undefined]

## Example

```typescript
import { PayrollCalculationResult } from './api';

const instance: PayrollCalculationResult = {
    employee_id,
    period_start,
    period_end,
    net_salary,
    gross_salary,
    total_deductions,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

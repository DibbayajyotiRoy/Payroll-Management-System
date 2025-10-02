# Employee

Represents a full employee record in the system.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **string** | The unique identifier for the employee. | [optional] [default to undefined]
**employee_id** | **string** | The human-readable employee ID. | [optional] [default to undefined]
**first_name** | **string** |  | [optional] [default to undefined]
**last_name** | **string** |  | [optional] [default to undefined]
**email** | **string** |  | [optional] [default to undefined]
**phone** | **string** |  | [optional] [default to undefined]
**hire_date** | **string** |  | [optional] [default to undefined]
**department** | **string** |  | [optional] [default to undefined]
**position** | **string** |  | [optional] [default to undefined]
**status** | **string** |  | [optional] [default to undefined]
**salary_info** | [**SalaryInfo**](SalaryInfo.md) |  | [optional] [default to undefined]
**created_at** | **string** |  | [optional] [default to undefined]
**updated_at** | **string** |  | [optional] [default to undefined]

## Example

```typescript
import { Employee } from './api';

const instance: Employee = {
    id,
    employee_id,
    first_name,
    last_name,
    email,
    phone,
    hire_date,
    department,
    position,
    status,
    salary_info,
    created_at,
    updated_at,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

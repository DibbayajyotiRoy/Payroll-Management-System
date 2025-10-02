# CreateEmployee

Payload for creating a new employee.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**first_name** | **string** |  | [default to undefined]
**last_name** | **string** |  | [default to undefined]
**email** | **string** |  | [default to undefined]
**department** | **string** |  | [default to undefined]
**position** | **string** |  | [default to undefined]
**salary_info** | [**SalaryInfo**](SalaryInfo.md) |  | [default to undefined]

## Example

```typescript
import { CreateEmployee } from './api';

const instance: CreateEmployee = {
    first_name,
    last_name,
    email,
    department,
    position,
    salary_info,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

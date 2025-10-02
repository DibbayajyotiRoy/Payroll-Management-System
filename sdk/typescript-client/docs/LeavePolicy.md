# LeavePolicy

Defines the rules for a specific type of leave.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**leave_type_id** | **string** | Unique identifier for the leave type (e.g., \&#39;SICK\&#39;, \&#39;UNPAID\&#39;). | [optional] [default to undefined]
**leave_type_name** | **string** | Human-readable name of the leave type. | [optional] [default to undefined]
**deduction** | [**LeavePolicyDeduction**](LeavePolicyDeduction.md) |  | [optional] [default to undefined]

## Example

```typescript
import { LeavePolicy } from './api';

const instance: LeavePolicy = {
    leave_type_id,
    leave_type_name,
    deduction,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

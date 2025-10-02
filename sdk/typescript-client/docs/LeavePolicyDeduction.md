# LeavePolicyDeduction

Defines how much salary is deducted for this leave type.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**type** | **string** | The type of deduction: \&#39;percent\&#39; of daily salary, \&#39;flat\&#39; amount, or \&#39;none\&#39;. | [optional] [default to undefined]
**value** | **number** | The value of the deduction. For \&#39;percent\&#39;, this is 0-100. For \&#39;flat\&#39;, it\&#39;s a monetary value. | [optional] [default to undefined]

## Example

```typescript
import { LeavePolicyDeduction } from './api';

const instance: LeavePolicyDeduction = {
    type,
    value,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

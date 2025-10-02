# OvertimePolicy

Rules defining multipliers for overtime pay.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**weekday_multiplier** | **number** |  | [optional] [default to undefined]
**weekend_multiplier** | **number** |  | [optional] [default to undefined]
**holiday_multiplier** | **number** |  | [optional] [default to undefined]

## Example

```typescript
import { OvertimePolicy } from './api';

const instance: OvertimePolicy = {
    weekday_multiplier,
    weekend_multiplier,
    holiday_multiplier,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

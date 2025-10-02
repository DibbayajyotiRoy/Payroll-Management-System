# LeavePolicyDeduction

Defines how much salary is deducted for this leave type.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**type** | **str** | The type of deduction: &#39;percent&#39; of daily salary, &#39;flat&#39; amount, or &#39;none&#39;. | [optional] 
**value** | **float** | The value of the deduction. For &#39;percent&#39;, this is 0-100. For &#39;flat&#39;, it&#39;s a monetary value. | [optional] 

## Example

```python
from openapi_client.models.leave_policy_deduction import LeavePolicyDeduction

# TODO update the JSON string below
json = "{}"
# create an instance of LeavePolicyDeduction from a JSON string
leave_policy_deduction_instance = LeavePolicyDeduction.from_json(json)
# print the JSON string representation of the object
print(LeavePolicyDeduction.to_json())

# convert the object into a dict
leave_policy_deduction_dict = leave_policy_deduction_instance.to_dict()
# create an instance of LeavePolicyDeduction from a dict
leave_policy_deduction_from_dict = LeavePolicyDeduction.from_dict(leave_policy_deduction_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



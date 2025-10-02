# LeavePolicy

Defines the rules for a specific type of leave.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**leave_type_id** | **str** | Unique identifier for the leave type (e.g., &#39;SICK&#39;, &#39;UNPAID&#39;). | [optional] 
**leave_type_name** | **str** | Human-readable name of the leave type. | [optional] 
**deduction** | [**LeavePolicyDeduction**](LeavePolicyDeduction.md) |  | [optional] 

## Example

```python
from openapi_client.models.leave_policy import LeavePolicy

# TODO update the JSON string below
json = "{}"
# create an instance of LeavePolicy from a JSON string
leave_policy_instance = LeavePolicy.from_json(json)
# print the JSON string representation of the object
print(LeavePolicy.to_json())

# convert the object into a dict
leave_policy_dict = leave_policy_instance.to_dict()
# create an instance of LeavePolicy from a dict
leave_policy_from_dict = LeavePolicy.from_dict(leave_policy_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



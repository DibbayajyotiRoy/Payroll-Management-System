# OvertimePolicy

Rules defining multipliers for overtime pay.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**weekday_multiplier** | **float** |  | [optional] 
**weekend_multiplier** | **float** |  | [optional] 
**holiday_multiplier** | **float** |  | [optional] 

## Example

```python
from openapi_client.models.overtime_policy import OvertimePolicy

# TODO update the JSON string below
json = "{}"
# create an instance of OvertimePolicy from a JSON string
overtime_policy_instance = OvertimePolicy.from_json(json)
# print the JSON string representation of the object
print(OvertimePolicy.to_json())

# convert the object into a dict
overtime_policy_dict = overtime_policy_instance.to_dict()
# create an instance of OvertimePolicy from a dict
overtime_policy_from_dict = OvertimePolicy.from_dict(overtime_policy_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



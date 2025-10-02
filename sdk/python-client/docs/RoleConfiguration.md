# RoleConfiguration


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**company_id** | **str** | The ID of the company this role belongs to. | 
**role_name** | **str** | The human-readable name of the role. | 
**schema_version** | **str** | The version of this configuration schema. | [optional] 
**base_salary_minor_units** | **int** | Base monthly salary in the smallest currency unit (e.g., cents) to avoid floating-point issues. | 
**currency** | **str** | The ISO 4217 currency code. | 
**overtime_policy** | [**OvertimePolicy**](OvertimePolicy.md) |  | 
**leave_policies** | [**List[LeavePolicy]**](LeavePolicy.md) | A list of policies for different types of leave. | 
**working_hours_per_day** | **float** |  | 
**working_days_per_week** | **int** |  | 
**is_active** | **bool** |  | 
**role_id** | **str** | Unique identifier for the role configuration, generated on creation. | [optional] 

## Example

```python
from openapi_client.models.role_configuration import RoleConfiguration

# TODO update the JSON string below
json = "{}"
# create an instance of RoleConfiguration from a JSON string
role_configuration_instance = RoleConfiguration.from_json(json)
# print the JSON string representation of the object
print(RoleConfiguration.to_json())

# convert the object into a dict
role_configuration_dict = role_configuration_instance.to_dict()
# create an instance of RoleConfiguration from a dict
role_configuration_from_dict = RoleConfiguration.from_dict(role_configuration_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



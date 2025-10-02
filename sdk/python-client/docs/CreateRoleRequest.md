# CreateRoleRequest

Payload for creating a new role configuration.

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

## Example

```python
from openapi_client.models.create_role_request import CreateRoleRequest

# TODO update the JSON string below
json = "{}"
# create an instance of CreateRoleRequest from a JSON string
create_role_request_instance = CreateRoleRequest.from_json(json)
# print the JSON string representation of the object
print(CreateRoleRequest.to_json())

# convert the object into a dict
create_role_request_dict = create_role_request_instance.to_dict()
# create an instance of CreateRoleRequest from a dict
create_role_request_from_dict = CreateRoleRequest.from_dict(create_role_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# CreateEmployee

Payload for creating a new employee.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**first_name** | **str** |  | 
**last_name** | **str** |  | 
**email** | **str** |  | 
**department** | **str** |  | 
**position** | **str** |  | 
**salary_info** | [**SalaryInfo**](SalaryInfo.md) |  | 

## Example

```python
from openapi_client.models.create_employee import CreateEmployee

# TODO update the JSON string below
json = "{}"
# create an instance of CreateEmployee from a JSON string
create_employee_instance = CreateEmployee.from_json(json)
# print the JSON string representation of the object
print(CreateEmployee.to_json())

# convert the object into a dict
create_employee_dict = create_employee_instance.to_dict()
# create an instance of CreateEmployee from a dict
create_employee_from_dict = CreateEmployee.from_dict(create_employee_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



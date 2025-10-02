# SalaryInfo


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**base_salary** | **float** | The base salary amount. | [optional] 
**currency** | **str** | ISO 4217 currency code. | [optional] 
**pay_frequency** | **str** |  | [optional] 
**overtime_rate** | **float** |  | [optional] 

## Example

```python
from openapi_client.models.salary_info import SalaryInfo

# TODO update the JSON string below
json = "{}"
# create an instance of SalaryInfo from a JSON string
salary_info_instance = SalaryInfo.from_json(json)
# print the JSON string representation of the object
print(SalaryInfo.to_json())

# convert the object into a dict
salary_info_dict = salary_info_instance.to_dict()
# create an instance of SalaryInfo from a dict
salary_info_from_dict = SalaryInfo.from_dict(salary_info_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



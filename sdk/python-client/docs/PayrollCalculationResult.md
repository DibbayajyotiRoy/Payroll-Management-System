# PayrollCalculationResult

Detailed results of a payroll calculation for a single employee.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**employee_id** | **str** |  | [optional] 
**period_start** | **date** |  | [optional] 
**period_end** | **date** |  | [optional] 
**net_salary** | **float** | The final take-home pay for the employee. | [optional] 
**gross_salary** | **float** | Total earnings before any deductions. | [optional] 
**total_deductions** | **float** | The sum of all deductions. | [optional] 

## Example

```python
from openapi_client.models.payroll_calculation_result import PayrollCalculationResult

# TODO update the JSON string below
json = "{}"
# create an instance of PayrollCalculationResult from a JSON string
payroll_calculation_result_instance = PayrollCalculationResult.from_json(json)
# print the JSON string representation of the object
print(PayrollCalculationResult.to_json())

# convert the object into a dict
payroll_calculation_result_dict = payroll_calculation_result_instance.to_dict()
# create an instance of PayrollCalculationResult from a dict
payroll_calculation_result_from_dict = PayrollCalculationResult.from_dict(payroll_calculation_result_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



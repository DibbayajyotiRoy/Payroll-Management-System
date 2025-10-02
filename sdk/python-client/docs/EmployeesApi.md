# openapi_client.EmployeesApi

All URIs are relative to *http://127.0.0.1:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**employees_get**](EmployeesApi.md#employees_get) | **GET** /employees | List All Employees
[**employees_id_get**](EmployeesApi.md#employees_id_get) | **GET** /employees/{id} | Get Employee by ID
[**employees_post**](EmployeesApi.md#employees_post) | **POST** /employees | Create a New Employee


# **employees_get**
> List[Employee] employees_get()

List All Employees

Retrieves a list of all employee records. This is a placeholder.

### Example


```python
import openapi_client
from openapi_client.models.employee import Employee
from openapi_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://127.0.0.1:8080
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "http://127.0.0.1:8080"
)


# Enter a context with an instance of the API client
with openapi_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = openapi_client.EmployeesApi(api_client)

    try:
        # List All Employees
        api_response = api_instance.employees_get()
        print("The response of EmployeesApi->employees_get:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling EmployeesApi->employees_get: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**List[Employee]**](Employee.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | A list of employees. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **employees_id_get**
> Employee employees_id_get(id)

Get Employee by ID

Retrieves a single employee by their unique ID. This is a placeholder.

### Example


```python
import openapi_client
from openapi_client.models.employee import Employee
from openapi_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://127.0.0.1:8080
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "http://127.0.0.1:8080"
)


# Enter a context with an instance of the API client
with openapi_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = openapi_client.EmployeesApi(api_client)
    id = 'a1b2c3d4-e5f6-7890-1234-567890abcdef' # str | The unique identifier of the employee.

    try:
        # Get Employee by ID
        api_response = api_instance.employees_id_get(id)
        print("The response of EmployeesApi->employees_id_get:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling EmployeesApi->employees_id_get: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **str**| The unique identifier of the employee. | 

### Return type

[**Employee**](Employee.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | The full employee object. |  -  |
**404** | Employee not found. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **employees_post**
> Employee employees_post(create_employee)

Create a New Employee

Creates a new employee record. This endpoint is a placeholder and does not yet connect to the database.

### Example


```python
import openapi_client
from openapi_client.models.create_employee import CreateEmployee
from openapi_client.models.employee import Employee
from openapi_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://127.0.0.1:8080
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "http://127.0.0.1:8080"
)


# Enter a context with an instance of the API client
with openapi_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = openapi_client.EmployeesApi(api_client)
    create_employee = openapi_client.CreateEmployee() # CreateEmployee | 

    try:
        # Create a New Employee
        api_response = api_instance.employees_post(create_employee)
        print("The response of EmployeesApi->employees_post:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling EmployeesApi->employees_post: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **create_employee** | [**CreateEmployee**](CreateEmployee.md)|  | 

### Return type

[**Employee**](Employee.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**201** | Employee created successfully. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


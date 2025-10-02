# EmployeesApi

All URIs are relative to *http://127.0.0.1:8080*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**employeesGet**](#employeesget) | **GET** /employees | List All Employees|
|[**employeesIdGet**](#employeesidget) | **GET** /employees/{id} | Get Employee by ID|
|[**employeesPost**](#employeespost) | **POST** /employees | Create a New Employee|

# **employeesGet**
> Array<Employee> employeesGet()

Retrieves a list of all employee records. This is a placeholder.

### Example

```typescript
import {
    EmployeesApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new EmployeesApi(configuration);

const { status, data } = await apiInstance.employeesGet();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**Array<Employee>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | A list of employees. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **employeesIdGet**
> Employee employeesIdGet()

Retrieves a single employee by their unique ID. This is a placeholder.

### Example

```typescript
import {
    EmployeesApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new EmployeesApi(configuration);

let id: string; //The unique identifier of the employee. (default to undefined)

const { status, data } = await apiInstance.employeesIdGet(
    id
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **id** | [**string**] | The unique identifier of the employee. | defaults to undefined|


### Return type

**Employee**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | The full employee object. |  -  |
|**404** | Employee not found. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **employeesPost**
> Employee employeesPost(createEmployee)

Creates a new employee record. This endpoint is a placeholder and does not yet connect to the database.

### Example

```typescript
import {
    EmployeesApi,
    Configuration,
    CreateEmployee
} from './api';

const configuration = new Configuration();
const apiInstance = new EmployeesApi(configuration);

let createEmployee: CreateEmployee; //

const { status, data } = await apiInstance.employeesPost(
    createEmployee
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **createEmployee** | **CreateEmployee**|  | |


### Return type

**Employee**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**201** | Employee created successfully. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


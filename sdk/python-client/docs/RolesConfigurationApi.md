# openapi_client.RolesConfigurationApi

All URIs are relative to *http://127.0.0.1:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**roles_get**](RolesConfigurationApi.md#roles_get) | **GET** /roles | List All Roles
[**roles_post**](RolesConfigurationApi.md#roles_post) | **POST** /roles | Create a New Role
[**roles_role_id_delete**](RolesConfigurationApi.md#roles_role_id_delete) | **DELETE** /roles/{role_id} | Delete a Role
[**roles_role_id_get**](RolesConfigurationApi.md#roles_role_id_get) | **GET** /roles/{role_id} | Get Role by ID
[**roles_role_id_put**](RolesConfigurationApi.md#roles_role_id_put) | **PUT** /roles/{role_id} | Update a Role


# **roles_get**
> List[RoleConfiguration] roles_get()

List All Roles

Retrieves a list of all role configurations in the system.

### Example


```python
import openapi_client
from openapi_client.models.role_configuration import RoleConfiguration
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
    api_instance = openapi_client.RolesConfigurationApi(api_client)

    try:
        # List All Roles
        api_response = api_instance.roles_get()
        print("The response of RolesConfigurationApi->roles_get:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RolesConfigurationApi->roles_get: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**List[RoleConfiguration]**](RoleConfiguration.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | A list of role configurations. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **roles_post**
> RoleConfiguration roles_post(create_role_request)

Create a New Role

Creates a new role configuration, defining salary, leave, and overtime policies for a group of employees.

### Example


```python
import openapi_client
from openapi_client.models.create_role_request import CreateRoleRequest
from openapi_client.models.role_configuration import RoleConfiguration
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
    api_instance = openapi_client.RolesConfigurationApi(api_client)
    create_role_request = {"company_id":"COMP001","role_name":"Senior Software Engineer","schema_version":"1.0","base_salary_minor_units":12000000,"currency":"USD","overtime_policy":{"weekday_multiplier":1.5,"weekend_multiplier":2.0,"holiday_multiplier":2.5},"leave_policies":[{"leave_type_id":"SICK","leave_type_name":"Sick Leave","deduction":{"type":"percent","value":0}},{"leave_type_id":"UNPAID","leave_type_name":"Unpaid Leave","deduction":{"type":"percent","value":100}}],"working_hours_per_day":8.0,"working_days_per_week":5,"is_active":true} # CreateRoleRequest | 

    try:
        # Create a New Role
        api_response = api_instance.roles_post(create_role_request)
        print("The response of RolesConfigurationApi->roles_post:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RolesConfigurationApi->roles_post: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **create_role_request** | [**CreateRoleRequest**](CreateRoleRequest.md)|  | 

### Return type

[**RoleConfiguration**](RoleConfiguration.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**201** | Role created successfully. Returns the newly created role configuration. |  -  |
**400** | Bad request due to invalid input data. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **roles_role_id_delete**
> roles_role_id_delete(role_id)

Delete a Role

Deletes a role configuration by its ID.

### Example


```python
import openapi_client
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
    api_instance = openapi_client.RolesConfigurationApi(api_client)
    role_id = 'role_a1b2c3d4' # str | The unique identifier for the role.

    try:
        # Delete a Role
        api_instance.roles_role_id_delete(role_id)
    except Exception as e:
        print("Exception when calling RolesConfigurationApi->roles_role_id_delete: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **role_id** | **str**| The unique identifier for the role. | 

### Return type

void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**204** | Role deleted successfully. |  -  |
**404** | Role with the specified ID not found. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **roles_role_id_get**
> RoleConfiguration roles_role_id_get(role_id)

Get Role by ID

Retrieves a specific role configuration by its ID.

### Example


```python
import openapi_client
from openapi_client.models.role_configuration import RoleConfiguration
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
    api_instance = openapi_client.RolesConfigurationApi(api_client)
    role_id = 'role_a1b2c3d4' # str | The unique identifier for the role.

    try:
        # Get Role by ID
        api_response = api_instance.roles_role_id_get(role_id)
        print("The response of RolesConfigurationApi->roles_role_id_get:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RolesConfigurationApi->roles_role_id_get: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **role_id** | **str**| The unique identifier for the role. | 

### Return type

[**RoleConfiguration**](RoleConfiguration.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | The requested role configuration. |  -  |
**404** | Role with the specified ID not found. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **roles_role_id_put**
> roles_role_id_put(role_id, role_configuration)

Update a Role

Updates an existing role configuration by its ID.

### Example


```python
import openapi_client
from openapi_client.models.role_configuration import RoleConfiguration
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
    api_instance = openapi_client.RolesConfigurationApi(api_client)
    role_id = 'role_a1b2c3d4' # str | The unique identifier for the role.
    role_configuration = openapi_client.RoleConfiguration() # RoleConfiguration | 

    try:
        # Update a Role
        api_instance.roles_role_id_put(role_id, role_configuration)
    except Exception as e:
        print("Exception when calling RolesConfigurationApi->roles_role_id_put: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **role_id** | **str**| The unique identifier for the role. | 
 **role_configuration** | [**RoleConfiguration**](RoleConfiguration.md)|  | 

### Return type

void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: Not defined

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Role updated successfully. |  -  |
**404** | Role with the specified ID not found. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


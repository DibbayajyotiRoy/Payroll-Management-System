# RolesConfigurationApi

All URIs are relative to *http://127.0.0.1:8080*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**rolesGet**](#rolesget) | **GET** /roles | List All Roles|
|[**rolesPost**](#rolespost) | **POST** /roles | Create a New Role|
|[**rolesRoleIdDelete**](#rolesroleiddelete) | **DELETE** /roles/{role_id} | Delete a Role|
|[**rolesRoleIdGet**](#rolesroleidget) | **GET** /roles/{role_id} | Get Role by ID|
|[**rolesRoleIdPut**](#rolesroleidput) | **PUT** /roles/{role_id} | Update a Role|

# **rolesGet**
> Array<RoleConfiguration> rolesGet()

Retrieves a list of all role configurations in the system.

### Example

```typescript
import {
    RolesConfigurationApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new RolesConfigurationApi(configuration);

const { status, data } = await apiInstance.rolesGet();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**Array<RoleConfiguration>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | A list of role configurations. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **rolesPost**
> RoleConfiguration rolesPost(createRoleRequest)

Creates a new role configuration, defining salary, leave, and overtime policies for a group of employees.

### Example

```typescript
import {
    RolesConfigurationApi,
    Configuration,
    CreateRoleRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new RolesConfigurationApi(configuration);

let createRoleRequest: CreateRoleRequest; //

const { status, data } = await apiInstance.rolesPost(
    createRoleRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **createRoleRequest** | **CreateRoleRequest**|  | |


### Return type

**RoleConfiguration**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**201** | Role created successfully. Returns the newly created role configuration. |  -  |
|**400** | Bad request due to invalid input data. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **rolesRoleIdDelete**
> rolesRoleIdDelete()

Deletes a role configuration by its ID.

### Example

```typescript
import {
    RolesConfigurationApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new RolesConfigurationApi(configuration);

let roleId: string; //The unique identifier for the role. (default to undefined)

const { status, data } = await apiInstance.rolesRoleIdDelete(
    roleId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **roleId** | [**string**] | The unique identifier for the role. | defaults to undefined|


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
|**204** | Role deleted successfully. |  -  |
|**404** | Role with the specified ID not found. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **rolesRoleIdGet**
> RoleConfiguration rolesRoleIdGet()

Retrieves a specific role configuration by its ID.

### Example

```typescript
import {
    RolesConfigurationApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new RolesConfigurationApi(configuration);

let roleId: string; //The unique identifier for the role. (default to undefined)

const { status, data } = await apiInstance.rolesRoleIdGet(
    roleId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **roleId** | [**string**] | The unique identifier for the role. | defaults to undefined|


### Return type

**RoleConfiguration**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | The requested role configuration. |  -  |
|**404** | Role with the specified ID not found. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **rolesRoleIdPut**
> rolesRoleIdPut(roleConfiguration)

Updates an existing role configuration by its ID.

### Example

```typescript
import {
    RolesConfigurationApi,
    Configuration,
    RoleConfiguration
} from './api';

const configuration = new Configuration();
const apiInstance = new RolesConfigurationApi(configuration);

let roleId: string; //The unique identifier for the role. (default to undefined)
let roleConfiguration: RoleConfiguration; //

const { status, data } = await apiInstance.rolesRoleIdPut(
    roleId,
    roleConfiguration
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **roleConfiguration** | **RoleConfiguration**|  | |
| **roleId** | [**string**] | The unique identifier for the role. | defaults to undefined|


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
|**200** | Role updated successfully. |  -  |
|**404** | Role with the specified ID not found. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


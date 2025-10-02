# PlaceholdersFutureFeaturesApi

All URIs are relative to *http://127.0.0.1:8080*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**analyticsReportsGet**](#analyticsreportsget) | **GET** /analytics/reports | Get Analytics Reports|
|[**payrollRunPost**](#payrollrunpost) | **POST** /payroll/run | Manually Trigger a Payroll Run|

# **analyticsReportsGet**
> analyticsReportsGet()

Fetches AI-powered analytics. This is a placeholder for a future feature.

### Example

```typescript
import {
    PlaceholdersFutureFeaturesApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new PlaceholdersFutureFeaturesApi(configuration);

const { status, data } = await apiInstance.analyticsReportsGet();
```

### Parameters
This endpoint does not have any parameters.


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
|**200** | Analytics report. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **payrollRunPost**
> payrollRunPost()

Manually initiates a payroll run. This is a placeholder for a future feature.

### Example

```typescript
import {
    PlaceholdersFutureFeaturesApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new PlaceholdersFutureFeaturesApi(configuration);

const { status, data } = await apiInstance.payrollRunPost();
```

### Parameters
This endpoint does not have any parameters.


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
|**202** | Payroll run accepted for processing. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# PayrollAutomationApi

All URIs are relative to *http://127.0.0.1:8080*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**payrollRunsRunIdGet**](#payrollrunsrunidget) | **GET** /payroll/runs/{run_id} | Get Payroll Run Status|

# **payrollRunsRunIdGet**
> payrollRunsRunIdGet()

Fetches the status and results of a specific automated payroll run.

### Example

```typescript
import {
    PayrollAutomationApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new PayrollAutomationApi(configuration);

let runId: string; //The UUID of the payroll run job. (default to undefined)

const { status, data } = await apiInstance.payrollRunsRunIdGet(
    runId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **runId** | [**string**] | The UUID of the payroll run job. | defaults to undefined|


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
|**200** | The status and details of the payroll run. |  -  |
|**404** | Payroll run with the specified ID not found. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


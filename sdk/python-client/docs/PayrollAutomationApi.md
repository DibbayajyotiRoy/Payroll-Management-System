# openapi_client.PayrollAutomationApi

All URIs are relative to *http://127.0.0.1:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**payroll_runs_run_id_get**](PayrollAutomationApi.md#payroll_runs_run_id_get) | **GET** /payroll/runs/{run_id} | Get Payroll Run Status


# **payroll_runs_run_id_get**
> payroll_runs_run_id_get(run_id)

Get Payroll Run Status

Fetches the status and results of a specific automated payroll run.

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
    api_instance = openapi_client.PayrollAutomationApi(api_client)
    run_id = 'f4g5h6j7-k8l9-1011-1213-141516abcdef' # str | The UUID of the payroll run job.

    try:
        # Get Payroll Run Status
        api_instance.payroll_runs_run_id_get(run_id)
    except Exception as e:
        print("Exception when calling PayrollAutomationApi->payroll_runs_run_id_get: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **run_id** | **str**| The UUID of the payroll run job. | 

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
**200** | The status and details of the payroll run. |  -  |
**404** | Payroll run with the specified ID not found. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


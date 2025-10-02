# openapi_client.PlaceholdersFutureFeaturesApi

All URIs are relative to *http://127.0.0.1:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**analytics_reports_get**](PlaceholdersFutureFeaturesApi.md#analytics_reports_get) | **GET** /analytics/reports | Get Analytics Reports
[**payroll_run_post**](PlaceholdersFutureFeaturesApi.md#payroll_run_post) | **POST** /payroll/run | Manually Trigger a Payroll Run


# **analytics_reports_get**
> analytics_reports_get()

Get Analytics Reports

Fetches AI-powered analytics. This is a placeholder for a future feature.

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
    api_instance = openapi_client.PlaceholdersFutureFeaturesApi(api_client)

    try:
        # Get Analytics Reports
        api_instance.analytics_reports_get()
    except Exception as e:
        print("Exception when calling PlaceholdersFutureFeaturesApi->analytics_reports_get: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

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
**200** | Analytics report. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **payroll_run_post**
> payroll_run_post()

Manually Trigger a Payroll Run

Manually initiates a payroll run. This is a placeholder for a future feature.

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
    api_instance = openapi_client.PlaceholdersFutureFeaturesApi(api_client)

    try:
        # Manually Trigger a Payroll Run
        api_instance.payroll_run_post()
    except Exception as e:
        print("Exception when calling PlaceholdersFutureFeaturesApi->payroll_run_post: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

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
**202** | Payroll run accepted for processing. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


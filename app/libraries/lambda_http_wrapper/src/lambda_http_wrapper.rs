pub use lambda_http::Error;
use lambda_http::{
    run as run_lambda, service_fn, Body, Request as LambdaRequest, Response as LambdaResponse,
};
use serde::{de::DeserializeOwned, Serialize};
use std::{future::Future, pin::Pin, sync::Arc};

/////////////////////////////////////////////////////
// 1. Shared Helper Functions
/////////////////////////////////////////////////////

/// Build a JSON HTTP response with the given status code from a serializable value.
fn build_json_response<T: Serialize>(
    status: u16,
    body_data: &T,
) -> Result<LambdaResponse<Body>, Error> {
    let body = serde_json::to_string(body_data).map_err(Error::from)?;
    let response = LambdaResponse::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(body.into())
        .map_err(Error::from)?;
    Ok(response)
}

/// Convert a Result from the controller into a proper HTTP response:
/// - On Ok, return 200 with the JSON–serialized value.
/// - On Err, return 400 with the JSON–serialized error.
fn result_to_http_response<TResp, TErr>(
    result: Result<TResp, TErr>,
) -> Result<LambdaResponse<Body>, Error>
where
    TResp: Serialize,
    TErr: Serialize,
{
    match result {
        Ok(success_val) => build_json_response(200, &success_val),
        Err(error_val) => build_json_response(400, &error_val),
    }
}

/// Attempt to parse the request body as JSON into TReq.
/// Returns:
/// - Ok(Some(parsed_value)) if the body is non-empty and successfully parsed.
/// - Ok(None) if the body is empty.
/// - Err(response) if the body is non-empty but invalid JSON, where `response`
///   is an HTTP 400 response.
fn parse_json_body<TReq: DeserializeOwned>(
    event: &LambdaRequest,
) -> Result<Option<TReq>, LambdaResponse<Body>> {
    let body_bytes = match event.body() {
        Body::Text(txt) => txt.as_bytes(),
        Body::Binary(bin) => bin,
        Body::Empty => &[],
    };

    if body_bytes.is_empty() {
        return Ok(None);
    }

    match serde_json::from_slice::<TReq>(body_bytes) {
        Ok(parsed) => Ok(Some(parsed)),
        Err(_err) => {
            let error_json = serde_json::json!({ "error": "Invalid JSON request body" });
            let response =
                build_json_response(400, &error_json).expect("build_json_response should not fail");
            Err(response)
        }
    }
}

/////////////////////////////////////////////////////
// 2. "With Input" Controller Wrapper and Run Function
/////////////////////////////////////////////////////

/// Wrap a controller that accepts an input of type `TReq`.
///
/// The controller has the signature:
///     async fn(TReq) -> Result<TResp, TErr>
///
/// The wrapper will:
///   1. Parse the Lambda event body as JSON into a `TReq`.
///   2. Call the controller with that value.
///   3. Convert the controller’s result into a JSON HTTP response (200 on success,
///      400 on error).
pub fn lambda_rest_controller<C, Fut, TReq, TResp, TErr>(
    controller: C,
) -> impl Fn(
    LambdaRequest,
) -> Pin<Box<dyn Future<Output = Result<LambdaResponse<Body>, Error>> + Send>>
       + Send
       + Sync
       + 'static
where
    C: Fn(TReq) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<TResp, TErr>> + Send + 'static,
    TReq: DeserializeOwned + 'static + Send,
    TResp: Serialize + 'static,
    TErr: Serialize + 'static,
{
    let arc_controller = Arc::new(controller);

    move |event: LambdaRequest| {
        let controller_clone = Arc::clone(&arc_controller);

        Box::pin(async move {
            // Attempt to parse the JSON body.
            match parse_json_body::<TReq>(&event) {
                Ok(Some(parsed_req)) => {
                    // Call the controller with the parsed input.
                    let result = controller_clone(parsed_req).await;
                    result_to_http_response(result)
                }
                Ok(None) => {
                    // Empty body: return a custom 400 response.
                    let error_json =
                        serde_json::json!({ "error": "Expected JSON body, got empty body" });
                    build_json_response(400, &error_json)
                }
                Err(response) => {
                    // Invalid JSON: return the already-built 400 response.
                    Ok(response)
                }
            }
        })
    }
}

/// A convenience function for running a controller that takes an input.
/// Initializes tracing, wraps the controller, and starts the Lambda runtime.
pub async fn run<C, Fut, TReq, TResp, TErr>(controller: C) -> Result<(), Error>
where
    C: Fn(TReq) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<TResp, TErr>> + Send + 'static,
    TReq: DeserializeOwned + 'static + Send,
    TResp: Serialize + 'static,
    TErr: Serialize + 'static,
{
    lambda_http::tracing::init_default_subscriber();
    run_lambda(service_fn(lambda_rest_controller(controller))).await
}

/////////////////////////////////////////////////////
// 3. "No Input" Controller Wrapper and Run Function
/////////////////////////////////////////////////////

/// Wrap a controller that accepts no input.
///
/// The controller has the signature:
///     async fn() -> Result<TResp, TErr>
///
/// The wrapper ignores the event body and simply calls the controller,
/// then converts its result into an HTTP response.
pub fn lambda_rest_controller_no_input<C, Fut, TResp, TErr>(
    controller: C,
) -> impl Fn(
    LambdaRequest,
) -> Pin<Box<dyn Future<Output = Result<LambdaResponse<Body>, Error>> + Send>>
       + Send
       + Sync
       + 'static
where
    C: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<TResp, TErr>> + Send + 'static,
    TResp: Serialize + 'static,
    TErr: Serialize + 'static,
{
    let arc_controller = Arc::new(controller);

    move |_event: LambdaRequest| {
        let controller_clone = Arc::clone(&arc_controller);
        Box::pin(async move {
            let result = controller_clone().await;
            result_to_http_response(result)
        })
    }
}

/// A convenience function for running a controller that takes no input.
/// Initializes tracing, wraps the controller, and starts the Lambda runtime.
pub async fn run_no_input<C, Fut, TResp, TErr>(controller: C) -> Result<(), Error>
where
    C: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<TResp, TErr>> + Send + 'static,
    TResp: Serialize + 'static,
    TErr: Serialize + 'static,
{
    lambda_http::tracing::init_default_subscriber();
    run_lambda(service_fn(lambda_rest_controller_no_input(controller))).await
}

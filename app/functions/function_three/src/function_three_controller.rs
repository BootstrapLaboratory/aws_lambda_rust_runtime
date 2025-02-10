use crate::function_three_types::*;

/// This is “business logic” controller
pub(crate) async fn function_three_controller() -> Result<Response, ErrorResponse> {
    // Here you would call into your service layer, etc.
    Ok(Response {
        message: "[Function_3] Hello function_three, this is an AWS Lambda HTTP request using controller wrapper to avoid lots of boilerplate".to_string(),
    })
}

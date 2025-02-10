use crate::function_one_types::*;

/// This is “business logic” controller
pub(crate) async fn function_one_controller(req: Request) -> Result<Response, ErrorResponse> {
    // For example, get a "name" from the query string (defaulting to "world")
    let name = req.name;

    // Here you would call into your service layer, etc.
    Ok(Response {
        message: format!("[Function_1] Hello {}, this is an AWS Lambda HTTP request using controller wrapper to avoid lots of boilerplate", name),
    })
}

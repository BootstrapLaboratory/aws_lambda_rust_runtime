# 2. Handle REST API requests (implement Lambda functions)

Before I implement my functions, I will add few dependencies to my project.

Cargo allows to manage dependencies versions in workspace `Cargo.toml`. It does not force me to use all of the dependencies I add to workspace in submodules, and I always can override versions in submodules. This approach is most flexible and provides clear benefits in long run. If I have large codebase with hundreds of submodules, I still have a central place for managing versions. Otherwise I have to update dependencies in each of my submodule, which can be painful. At the same time, if submodule requires a specific version for one of dependency, I can easily override it.

`Cargo.toml` (Workspace):

```toml
...

[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
lambda_runtime = "0.13.0"
tokio = { version = "1.0", features = ["macros"] }
```

`Cargo.toml` (functions/libraries):

```toml
...

[dependencies]
serde = { workspace = true }
serde_json = { workspace = true }
lambda_runtime = { workspace = true }
tokio = { workspace = true }
```

In [Lesson 1](01_initial_setup.md) I created my function submodules with `cargo new ...` command. I did it to create super lightweight setup and define project structure. `cargo-lambda` has it's own command to create lambda projects: `cargo lambda new ...`. Read docs about this command on documentation website - [cargo lambda new](https://www.cargo-lambda.info/commands/new.html). Use it, when it fit your needs. I will quickly show now, why you not always want to use it, and simple `cargo new ...` might make more sense.

Let's create a temporary project with this cargo lambda command. Inside `app/functions` directory (or inside some temp directory you like; anyways we will delete result of this command shortly):

```bash
cargo lambda new new-lambda-project
```

Check what is inside. It's very descriptive, with readme. It is using `lambda_http` to handle http requests, which is usually needed to implement regular REST/GraphQL service on server side. Read it all to understand what is going on.

I will copy everything which is inside `main.rs` and `http_handler.rs` inside my `function_one` submodule. And I also need dependencies. But remember that I use workspace wide dependencies to control its versions. And I already have something there. So I compare what is missing and see, that I need to add only one line in workspace `Cargo.toml`:

```toml
...
lambda_http = "0.14.0"
...
```

And my dependencies section in `app/functions/function_one/Cargo.toml` become this:

```toml
...
[dependencies]
lambda_http = { workspace = true }
tokio = { workspace = true }
...
```

`app/functions/function_one/src/function_one.rs`:

```rust
use lambda_http::{run, service_fn, tracing, Error};
mod http_handler;
use http_handler::function_handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
```

`app/functions/function_one/src/http_handler.rs`:

```rust
use lambda_http::{Body, Error, Request, RequestExt, Response};

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");
    let message = format!("Hello {who}, this is an AWS Lambda HTTP request");

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp: Response<Body> = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::{Request, RequestExt};
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_generic_http_handler() {
        let request = Request::default();

        let response = function_handler(request).await.unwrap();
        assert_eq!(response.status(), 200);

        let body_bytes = response.body().to_vec();
        let body_string = String::from_utf8(body_bytes).unwrap();

        assert_eq!(
            body_string,
            "Hello world, this is an AWS Lambda HTTP request"
        );
    }

    #[tokio::test]
    async fn test_http_handler_with_query_string() {
        let mut query_string_parameters: HashMap<String, String> = HashMap::new();
        query_string_parameters.insert("name".into(), "new-lambda-project".into());

        let request = Request::default().with_query_string_parameters(query_string_parameters);

        let response = function_handler(request).await.unwrap();
        assert_eq!(response.status(), 200);

        let body_bytes = response.body().to_vec();
        let body_string = String::from_utf8(body_bytes).unwrap();

        assert_eq!(
            body_string,
            "Hello new-lambda-project, this is an AWS Lambda HTTP request"
        );
    }
}
```

From this point I still can build my project with regular `cargo build`, but already cannot run it with commands from Lesson 1. I need to use now special commands from `cargo-lamba`, but about this a little bit later.

So now I want to make my second function. And I will not use the code example provided by `cargo lambda new...`, but I put there something else:

`app/functions/function_two/Cargo.toml`

```toml
[dependencies]
serde = { workspace = true }
serde_json = { workspace = true }
lambda_runtime = { workspace = true }
aws_lambda_events = { workspace = true }
tokio = { workspace = true }
```

`app/functions/function_two/src/function_two.rs`

```rust
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    #[serde(default)]
    message: String,
}

#[derive(Serialize)]
struct Response {
    req_message: String,
    my_response: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let (request, _context) = event.into_parts();
    Ok(Response {
        req_message: request.message,
        my_response: "Hello from Function One!".to_string(),
    })
}
```

Both examples use Rust for AWS Lambda, but they take different approaches. The first example aligns more closely with Cargo Lambda's recommended way of handling AWS Lambda functions when using HTTP requests (lambda_http). The second example follows a more generic event-driven Lambda model using lambda_runtime, which is also a valid approach but does not take advantage of the HTTP handling utilities provided by lambda_http.

- The first example uses lambda_http, which integrates directly with API Gateway and Lambda Function URLs.
- `function_two` uses `lambda_runtime`, which is a more general-purpose handler for AWS Lambda events.

Key Differences:

| Feature                    | First Example (`lambda_http`)                                      | Second Example (`lambda_runtime`)                                  |
| -------------------------- | ------------------------------------------------------------------ | ------------------------------------------------------------------ |
| **Library Used**           | `lambda_http`                                                      | `lambda_runtime`                                                   |
| **Handler Type**           | HTTP request handler                                               | Event-based handler                                                |
| **Request Processing**     | Extracts query params (`name`) and constructs an HTTP response     | Uses `serde` to deserialize JSON input and serialize output        |
| **Use Case**               | Best for Lambda functions exposed via API Gateway or Function URLs | Best for event-driven functions (SNS, SQS, DynamoDB Streams, etc.) |
| **Cargo Lambda Alignment** | Matches Cargo Lambda's HTTP handling best practices                | More generic, but still compatible with Cargo Lambda               |

And that is what I was talking about, that `cargo lambda new...` is not always what you always need.

I will remove now `new-lambda-project`, cause I copied from there all I need.

Let's go and build it:

```bash
cargo lambda build
```

Result is stores in `target/lambda` directory. Not very useful, cause I cannot simply run it. It requires AWS Lambda environment to operate. I will use `cargo-lambda` to run it locally:

```bash
$ cargo lambda watch
INFO starting Runtime server runtime_addr=[::]:9000
```

Server is running on `localhost:9000`. I will go and try to test it.

```bash
$ curl http://localhost:9000
{"detail":"the default function route is disabled, use /lambda-url/:function_name to trigger a function call. Available functions: {\"function_three\", \"function_one\", \"function_two\"}","title":"Default function disabled"}
```

Cool. It says, that I have three functions and I can access them by this url - `/lambda-url/:function_name`. I will try to access them now:

```bash
$ curl http://localhost:9000/lambda-url/function_one
Hello world, this is an AWS Lambda HTTP request

...

$ curl http://localhost:9000/lambda-url/function_two
{"type":"https://httpstatuses.com/500","status":500,"title":"Internal Server Error"}

...

$ curl http://localhost:9000/lambda-url/function_three
... just stuck :)
```

So. What I have here. `cargo lambda watch` prepares environment, but it does not build all the functions. Instead it waits, when you start accessing them and builds each of them on demand. So first access can be slow. Second access is very fast (about 10ms). It's very cool. Especially for large codebase.

As you can guess by command name, it also watching for changes in the source code and rebuilds those, which have been changed.

You can try different approaches. I run this command in workspace root, but it also can be run from submodule directories separately. I find it useful to run from workspace root. But it really depends on your need.

I'm not limited to use `curl` program. I can use any http client: special programs like Postman, or simply web browser.

`function_two` responds with an error. It expects specially prepared request object. I will not talk about it right now, cause I concentrate in REST API here. `cargo-lambda` have `invoke` command and plenty of examples how to test internal event driven Lambda functions.

A few words about `function_three`. It was also build by `cargo-lambda`. But it works very strange. First, I see "Hello world" message in terminal, where I run `cargo lambda watch`. And second, it stuck, when I try to access it with `curl`.

I want to fix it, so all my functions work well. I simply copy paste the code and config from `function_one`, and modify message to identify it as `function_three`. Not going to share source code, cause it is just copy/paste. Let's access it now:

```bash
$ curl http://localhost:9000/lambda-url/function_three
Hello function_three, this is an AWS Lambda HTTP request
```

`Hello function_three` it says. No need to restart anything. It works very well. So as you can notice `cargo-lambda` fits very well in monorepo projects and everything works smooth. Cargo itself has built-in support for monorepo development, no need to install and configure third-party plugins. And if you do not like monorepo approach, you are not limited. You can build and use `cargo-lambda` within one module, still having multiply functions build separately with its own dependencies. Matter of configurations. I like myself develop in monorepo, cause it gives clearer view on project structure and dependencies of each of the functions, and later I can reuse my modules easier.

So. in the end of the Lesson 2 I want to make small improvement of what I already have. I do not like how much boilerplate code I need to write to have simple REST API controller.

So I will create "rest api controller" wrapper for my Lambda functions and will use it, instead directly calling to `lambda_http`. So I finally will start using library that I have.

For first I want to rename it. `common_lib` is a bad name, as I mentioned already in Lesson 1. So I will name it `lambda_http_wrapper`, as it says better about what it provides.

Not going to write here all steps. Basically I need to rename library directory name, workspace `Cargo.toml` to point to it and `Cargo.toml` of library itself.

Below is one way to “wrap” my Lambda HTTP boilerplate. In this example I will:

- Define my own API types in a dedicated module (api) called Request, Response and ErrorResponse.
- Write a conversion from the raw lambda_http::Request into my own api::Request.
- Provide a helper function (wrap_controller) that accepts any async “controller” function with the signature

```rust
async fn controller(req: Request) -> Result<Response, ErrorResponse>
```

and returns a function ready for AWS Lambda’s runtime (with the proper status codes and JSON headers).

The end result is that your controller logic is “pure” (only using your API types) while all the Lambda–HTTP plumbing is done once in the wrapper.

Now I will paste here implementation of my new wrapper library:

`app/libraries/lambda_http_wrapper/Cargo.toml`

```toml
[package]
name = "lambda_http_wrapper"
version = "0.1.0"
edition = "2021"

[lib]
name = "lambda_http_wrapper"
path = "src/lambda_http_wrapper.rs"

[dependencies]
lambda_http = { workspace = true }
tokio = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
```

`app/libraries/lambda_http_wrapper/src/lambda_http_wrapper.rs`

```rust
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
```

Now I will modify my `function_one` and `function_three` to start using this wrapper (remove all the files and make everything from scratch):

`app/functions/function_one/Cargo.toml`

```toml
[package]
name = "function_one"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "function_one"
path = "src/function_one.rs"

[dependencies]
tokio = { workspace = true }
serde = { workspace = true }

# internal dependencies
lambda_http_wrapper = { path = "../../libraries/lambda_http_wrapper"}
```

`app/functions/function_one/src/function_one.rs`

```rust
use lambda_http_wrapper::Error;

use lambda_http_wrapper::run;

mod function_one_controller;
use function_one_controller::function_one_controller;
mod function_one_types;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(function_one_controller).await
}
```

`app/functions/function_one/src/function_one_types.rs`

```rust
//! This module defines the “API types” that your controllers use.

use serde::{Deserialize, Serialize};

/// Your application’s Request – you can add more fields as needed (e.g. headers, body).
#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    /// For simplicity we extract just the query parameters.
    pub name: String,
}

/// Your successful Response.
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub message: String,
}

/// Your Error Response.
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}
```

`app/functions/function_one/src/function_one_controller.rs`

```rust
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
```

Same for `function_three`, with few exceptions: I will make it run without request parameter, so it does not require `Request` type in `function_three_types.rs`. And because of this it uses `lambda_http_wrapper::run_no_input`. I will not paste here its code. It is almost identical. Check repository code for this.

It builds and works about the same like it already did without this wrapper.

I implemented first two hello-world functions that work like HTTP REST API handlers running that run locally, but can be deployed to AWS Lambda already. Also I created a wrapper for `lambda_http` to hide lots of boilerplate in library and make my Lambda functions code more readable.

`function_one` requires request parameter, `function_three` works without input parameter. I will try to call it:

```bash
$ curl http://localhost:9000/lambda-url/function_one
{"error":"Expected JSON body, got empty body"}

$ curl http://localhost:9000/lambda-url/function_one -d '{"name":"function_one"}' -H 'Content-Type: application/json'
{"message":"[Function_1] Hello function_one, this is an AWS Lambda HTTP request using controller wrapper to avoid lots of boilerplate"}

$ curl http://localhost:9000/lambda-url/function_three
{"message":"[Function_3] Hello function_three, this is an AWS Lambda HTTP request using controller wrapper to avoid lots of boilerplate"}
```

Wrapper works very well. I get readable error, when I use `function_one` without input parameter. Thanks to `gpt-o3-mini-hight` implementing it.

I think that just enough for Lesson 2.

[Browse the code at Lesson 2 checkpoint](https://github.com/BootstrapLaboratory/aws_lambda_rust_runtime/tree/lesson-2)

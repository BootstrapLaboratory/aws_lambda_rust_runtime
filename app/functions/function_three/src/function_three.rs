use lambda_http_wrapper::Error;

use lambda_http_wrapper::run_no_input as run;

mod function_three_controller;
use function_three_controller::function_three_controller;
mod function_three_types;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(function_three_controller).await
}

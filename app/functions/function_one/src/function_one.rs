use lambda_http_wrapper::Error;

use lambda_http_wrapper::run;

mod function_one_controller;
use function_one_controller::function_one_controller;
mod function_one_types;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(function_one_controller).await
}

use lambda_http_wrapper::Error;

use lambda_http_wrapper::run;

mod controller;
use controller::handle;
mod types;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handle).await
}

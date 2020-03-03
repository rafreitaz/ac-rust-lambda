use lambda_runtime::{error::HandlerError, lambda, Context};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(api_gateway_handler);

    Ok(())
}

pub fn api_gateway_handler(_request: String, _c: Context) -> Result<String, HandlerError> {
    let response = "{\"statusCode\": 200}".to_string();

    Ok(response)
}

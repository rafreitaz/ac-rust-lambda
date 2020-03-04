mod lambda_api;

use lambda_runtime::{error::HandlerError, lambda, Context};
use std::error::Error;
use serde_json;
use lambda_api::*;

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(api_gateway_handler);

    Ok(())
}

pub fn api_gateway_handler(request: ApiRequest, _c: Context) -> Result<ApiResponse, HandlerError> {
    let input = request
        .path_parameters
        .unwrap()
        .get("name")
        .unwrap()
        .clone();
    let response = ApiResponse {
        status_code: 200,
        body: serde_json::to_string(&EchoResponse { name: input }).unwrap(),
        ..Default::default()
    };
    Ok(response)
}



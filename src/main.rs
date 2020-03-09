extern crate log;

mod lambda_api;

use lambda_api::*;
use lambda_runtime::{error::HandlerError, lambda, Context};
use serde_json;
use simple_logger;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(api_gateway_handler);
    Ok(())
}

pub fn api_gateway_handler(request: ApiRequest, _c: Context) -> Result<ApiResponse, HandlerError> {
    let body: RequestBody = serde_json::from_str(&request.body).unwrap_or_else(RequestBody {
        name: String::from("Default"),
        age: 0,
    });

    log::info!("{}", &body.name);

    let response = ApiResponse {
        status_code: 200,
        body: request.body,
        ..Default::default()
    };
    Ok(response)
}

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
    let body: RequestBody = serde_json::from_str(&request.body).unwrap_or(RequestBody {
        ..Default::default()
    });

    log::info!("{}", &body.name);

    let response = ApiResponse {
        status_code: 200,
        body: serde_json::to_string(&body)?,
        ..Default::default()
    };
    Ok(response)
}

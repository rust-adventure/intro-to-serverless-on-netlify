use lambda_http::{
    http::header::CONTENT_TYPE, run, service_fn, Body,
    Error, Request, RequestExt, Response,
};
use serde::Serialize;

#[derive(Serialize)]
struct ApiResponse {
    data: String,
}

async fn function_handler(
    event: Request,
) -> Result<Response<Body>, Error> {
    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");

    let message = format!(
            "Hello {who}, this is an Netlify serverless request"
        );
    let api_response = ApiResponse { data: message };
    let body_text = serde_json::to_string(&api_response)?;

    let resp = Response::builder()
        .status(200)
        .header(CONTENT_TYPE, "application/json")
        .body(Body::Text(body_text))?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}

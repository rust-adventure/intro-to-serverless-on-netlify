use html_escape::encode_text;
use lambda_http::{
    http::header::CONTENT_TYPE, run, service_fn, Body,
    Error, Request, RequestExt, Response,
};

async fn function_handler(
    event: Request,
) -> Result<Response<Body>, Error> {
    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");

    let html = format!(
        "<html><body><h1>hello {}!</h1></body></html>",
        encode_text(who)
    );
    let resp = Response::builder()
        .status(200)
        .header(CONTENT_TYPE, "text/html")
        .body(Body::Text(html))?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}

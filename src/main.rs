use lambda_http::{service_fn, Error, RequestExt, IntoResponse, Request};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::run(service_fn(hello)).await?;
    Ok(())
}

async fn hello(
    request: Request
) -> Result<impl IntoResponse, std::convert::Infallible> {
    let _context = request.lambda_context_ref();

    Ok(format!(
        "hello {}",
        request
            .query_string_parameters_ref()
            .and_then(|params| params.first("name"))
            .unwrap_or_else(|| "stranger")
    ))
}
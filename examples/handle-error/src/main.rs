//! 错误处理示例。

use puzz::http::StatusCode;
use puzz::response::IntoResponse;
use puzz::service::ServiceExt;
use puzz::{middleware, service_fn, BoxError, Request, Response, Server};

#[tokio::main]
async fn main() {
    Server::new(|| service_fn(hello).with(middleware::handle_error(handle)))
        .bind(([127, 0, 0, 1], 80))
        .run()
        .await
        .unwrap();
}

async fn hello(_: Request) -> Result<Response, BoxError> {
    Err("An error occurred in the service".into())
}

/// 错误处理
fn handle(err: BoxError) -> Response {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
}

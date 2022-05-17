//! 路由示例。

use std::convert::Infallible;
use std::future::Future;

use puzz::http::StatusCode;
use puzz::response::IntoResponse;
use puzz::service::{Service, ServiceExt};
use puzz::{middleware, route, service_fn, BoxError, Request, Response, Router, Server};

#[tokio::main]
async fn main() {
    Server::new(|| app())
        .bind(([127, 0, 0, 1], 80))
        .run()
        .await
        .unwrap();
}

fn app() -> impl Service<
    Request,
    Response = Response,
    Error = Infallible,
    Future = impl Future<Output = Result<Response, Infallible>>,
> {
    Router::new()
        .route(
            "/a/1",
            route::get(service_fn(|_| async { Ok::<_, Infallible>("1") })),
        )
        .route(
            "/a/2",
            route::get(service_fn(|_| async { Ok::<_, Infallible>("2") })),
        )
        .route(
            "/a/:n",
            route::get(service_fn(|_| async { Ok::<_, Infallible>("3") })),
        )
        .route(
            "/b/*",
            route::post(service_fn(|_| async { Ok::<_, Infallible>("4") })),
        )
        .nest(
            "/c",
            route::post(service_fn(|_| async { Ok::<_, Infallible>("5") })),
        )
        // 错误处理
        .with(middleware::handle_error(|err: BoxError| {
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }))
}

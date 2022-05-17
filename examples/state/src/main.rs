//! 服务共享状态示例。

use std::convert::Infallible;
use std::future::Future;
use std::sync::Arc;

use puzz::response::IntoResponse;
use puzz::service::{Service, ServiceExt};
use puzz::{middleware, service_fn, Request, Server};
use tokio::sync::Mutex;

#[derive(Clone, Default)]
struct State {
    count: Arc<Mutex<u64>>,
}

#[tokio::main]
async fn main() {
    let state = State::default();

    Server::new(move || app(state.clone()))
        .bind(([127, 0, 0, 1], 80))
        .run()
        .await
        .unwrap();
}

fn app<T>(
    state: T,
) -> impl Service<
    Request,
    Response = impl IntoResponse,
    Error = Infallible,
    Future = impl Future<Output = Result<impl IntoResponse, Infallible>>,
>
where
    T: Clone + 'static,
{
    service_fn(count)
        // 将状态添加到请求扩展中
        .with(middleware::add_extension(move || state.clone()))
}

async fn count(request: Request) -> Result<impl IntoResponse, Infallible> {
    // 从请求扩展取出状态
    let state = puzz::extract::extension::<State>(&request).unwrap();

    let mut count = state.count.lock().await;

    *count += 1;

    Ok(format!("count: {}", *count))
}

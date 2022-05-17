//! 服务器推送事件示例。

use std::convert::Infallible;

use puzz::response::sse::{Event, KeepAlive};
use puzz::response::{self, IntoResponse};
use puzz::{service_fn, Request, Server};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    Server::new(|| service_fn(sse))
        .bind(([127, 0, 0, 1], 80))
        .run()
        .await
        .unwrap();
}

async fn sse(_: Request) -> Result<impl IntoResponse, Infallible> {
    let stream = futures_util::stream::repeat_with(|| Event::default().data("hi!"))
        .map(Ok::<_, Infallible>)
        .throttle(std::time::Duration::from_secs(3));

    Ok(response::sse(stream).keep_alive(KeepAlive::default()))
}

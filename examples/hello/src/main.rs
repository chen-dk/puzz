//! 一个简单的“hello world”示例。

use puzz::{service_fn, Server};

#[tokio::main]
async fn main() {
    Server::new(|| service_fn(|_| async { Ok("Hello, World!!!") }))
        .bind(([127, 0, 0, 1], 80))
        .run()
        .await
        .unwrap();
}

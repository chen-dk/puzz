<h1 align="center">
    Puzz
</h1>

<p align="center">
    一个简单且强大的网络框架
</p>

## 例子

```rust
use puzz::{service_fn, Server};

#[tokio::main]
async fn main() {
    Server::new(|| service_fn(|_| async { Ok("Hello, World!!!") }))
        .bind(([127, 0, 0, 1], 80))
        .run()
        .await
        .unwrap();
}
```

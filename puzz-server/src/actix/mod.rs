use std::convert::Infallible;
use std::fmt;
use std::net::SocketAddr;

use actix_http::HttpService;
use actix_service::IntoService;
use puzz_core::response::IntoResponse;
use puzz_core::service::Service;
use puzz_core::{BoxError, Request};
use tokio::net::TcpStream;

mod compat;

struct ServerOptions {
    workers: Option<usize>,
    addr: Vec<SocketAddr>,
}

/// HTTP服务器
///
/// # 例子
///
/// ```ignore
/// use puzz_core::service_fn;
/// use puzz_server::Server;
///
/// Server::new(|| service_fn(|_| async { Ok("hi!") }))
///     .bind(([127, 0, 0, 1], 80))
///     .run()
///     .await
///     .unwrap();
/// ```
pub struct Server<F> {
    factory: F,
    options: Result<ServerOptions, BoxError>,
}

impl<F, S> Server<F>
where
    F: Fn() -> S + Clone + Send + 'static,
    S: Service<Request, Error = Infallible> + 'static,
    S::Response: IntoResponse,
    S::Future: 'static,
{
    /// 创建一个新的HTTP服务器。
    pub fn new(factory: F) -> Self {
        Self {
            factory,
            options: Ok(ServerOptions {
                workers: None,
                addr: vec![],
            }),
        }
    }

    /// 设置HTTP服务器的工作线程数。
    ///
    /// 服务器的工作线程数默认设置为处理器的物理内核数。
    pub fn workers(mut self, num: usize) -> Self {
        self.options = self.options.and_then(|mut options| {
            options.workers = Some(num);
            Ok(options)
        });
        self
    }

    /// 设置HTTP服务器的监听地址。
    pub fn bind<A>(mut self, addr: A) -> Self
    where
        A: Into<SocketAddr>,
    {
        self.options = self.options.and_then(|mut options| {
            options.addr.push(addr.into());
            Ok(options)
        });
        self
    }

    /// 启动HTTP服务器。
    pub async fn run(self) -> Result<(), BoxError> {
        let options = self.options?;
        let factory = self.factory;

        let factory = move || {
            let service = compat::into_actix_service(factory());
            let service = move |request: actix_http::Request| service.call(request);

            async move { Ok::<_, Infallible>(service.into_service()) }
        };

        let mut server = actix_server::Server::build();

        if let Some(workers) = options.workers {
            server = server.workers(workers);
        }

        server
            .bind("puzz", &options.addr[..], move || {
                HttpService::<TcpStream, _, _, _, _>::build()
                    .finish(factory.clone())
                    .tcp()
            })?
            .run()
            .await
            .map_err(From::from)
    }
}

impl<F> fmt::Debug for Server<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Server").finish()
    }
}

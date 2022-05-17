use std::collections::HashMap;
use std::fmt;

use matchit::Match;
use puzz_core::http::uri::{Parts, PathAndQuery, Uri};
use puzz_core::response::IntoResponse;
use puzz_core::service::util::BoxService;
use puzz_core::service::{Service, ServiceExt};
use puzz_core::{BoxError, Request, Response};

use crate::error::NotFound;
use crate::RouteFuture;

const PRIVATE_TAIL_PARAM: &'static str = "__private__tail_param";

enum Endpoint {
    Full(BoxService<Request, Response, BoxError>),
    Nest(BoxService<Request, Response, BoxError>),
}

/// 路由器
///
/// 匹配传入的HTTP请求并将它们分派给[服务](`Service`)进行处理。
///
/// # 例子
///
/// ```
/// use std::convert::Infallible;
///
/// use puzz_core::service_fn;
/// use puzz_route::Router;
///
/// Router::new()
///     .route("/hi", service_fn(|_| async { Ok::<_, Infallible>("hi!") }));
/// ```
pub struct Router {
    inner: matchit::Router<Endpoint>,
}

impl Router {
    /// 创建一个空的路由器。
    pub fn new() -> Self {
        Self {
            inner: matchit::Router::new(),
        }
    }

    /// 将服务挂载到一条路由上。
    ///
    /// # 例子
    ///
    /// ```
    /// use std::convert::Infallible;
    ///
    /// use puzz_core::service_fn;
    /// use puzz_route::Router;
    ///
    /// Router::new()
    ///     .route("/hi", service_fn(|_| async { Ok::<_, Infallible>("hi!") }));
    /// ```
    pub fn route<S>(self, path: &str, service: S) -> Self
    where
        S: Service<Request> + 'static,
        S::Response: IntoResponse,
        S::Error: Into<BoxError>,
    {
        if !path.starts_with('/') {
            panic!("Path must start with a `/`");
        }
        let path = if path.ends_with('*') {
            format!("{path}{PRIVATE_TAIL_PARAM}")
        } else {
            path.into()
        };
        self.add_route(path, Endpoint::Full(Self::into_box_service(service)))
    }

    /// 将服务挂载到一条嵌套路由上。
    ///
    /// # 例子
    ///
    /// ```
    /// use std::convert::Infallible;
    ///
    /// use puzz_core::service_fn;
    /// use puzz_route::Router;
    ///
    /// Router::new()
    ///     .nest("/hi", service_fn(|_| async { Ok::<_, Infallible>("hi!") }));
    /// ```
    pub fn nest<S>(self, path: &str, service: S) -> Self
    where
        S: Service<Request> + 'static,
        S::Response: IntoResponse,
        S::Error: Into<BoxError>,
    {
        if !path.starts_with('/') {
            panic!("Path must start with a `/`");
        }
        let path = if path.ends_with('/') {
            format!("{path}*{PRIVATE_TAIL_PARAM}")
        } else {
            format!("{path}/*{PRIVATE_TAIL_PARAM}")
        };
        self.add_route(path, Endpoint::Nest(Self::into_box_service(service)))
    }

    fn add_route(mut self, path: String, endpoint: Endpoint) -> Self {
        if let Err(e) = self.inner.insert(path, endpoint) {
            panic!("{e}");
        }
        self
    }

    fn into_box_service<S>(service: S) -> BoxService<Request, Response, BoxError>
    where
        S: Service<Request> + 'static,
        S::Response: IntoResponse,
        S::Error: Into<BoxError>,
    {
        service
            .map_response(IntoResponse::into_response)
            .map_err(Into::into)
            .boxed()
    }
}

impl fmt::Debug for Router {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Router").finish()
    }
}

impl Service<Request> for Router {
    type Response = Response;
    type Error = BoxError;
    type Future = RouteFuture;

    fn call(&self, mut request: Request) -> Self::Future {
        match self.inner.at(request.uri().path()) {
            Ok(Match { value, params }) => {
                let fut = match value {
                    Endpoint::Full(service) => {
                        let (params, _) = take_params(params);
                        insert_params(&mut request, params);
                        service.call(request)
                    }
                    Endpoint::Nest(service) => {
                        let (params, tail) = take_params(params);
                        insert_params(&mut request, params);
                        replace_path(&mut request, &tail.unwrap());
                        service.call(request)
                    }
                };
                RouteFuture::Future { fut }
            }
            Err(_) => RouteFuture::Error {
                err: Some(NotFound::new(request).into()),
            },
        }
    }
}

/// 路由器提取的路径参数。
#[derive(Debug, Clone)]
pub struct Params(HashMap<String, String>);

impl Params {
    pub(crate) fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get_ref(&self) -> &HashMap<String, String> {
        &self.0
    }

    pub fn into_inner(self) -> HashMap<String, String> {
        self.0
    }
}

fn take_params(params: matchit::Params) -> (Vec<(String, String)>, Option<String>) {
    let mut path = None;
    (
        params
            .iter()
            .filter_map(|(k, v)| {
                if k == PRIVATE_TAIL_PARAM {
                    path = Some(v.to_owned());
                    None
                } else {
                    Some((k.to_owned(), v.to_owned()))
                }
            })
            .collect(),
        path,
    )
}

fn insert_params(request: &mut Request, captures: Vec<(String, String)>) {
    let extensions = request.extensions_mut();

    let params = if let Some(params) = extensions.get_mut::<Params>() {
        params
    } else {
        extensions.insert(Params::new());
        extensions.get_mut::<Params>().unwrap()
    };

    params.0.extend(captures);
}

fn replace_path(request: &mut Request, path: &str) {
    let uri = request.uri_mut();

    let path_and_query = if let Some(query) = uri.query() {
        format!("{}?{}", path, query)
            .parse::<PathAndQuery>()
            .unwrap()
    } else {
        path.parse().unwrap()
    };

    replace_path_and_query(uri, path_and_query);
}

fn replace_path_and_query(uri: &mut Uri, path_and_query: PathAndQuery) {
    let mut parts = Parts::default();

    parts.scheme = uri.scheme().cloned();
    parts.authority = uri.authority().cloned();
    parts.path_and_query = Some(path_and_query);

    *uri = Uri::from_parts(parts).unwrap();
}

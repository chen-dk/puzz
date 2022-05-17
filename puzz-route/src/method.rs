use puzz_core::http::Method;
use puzz_core::response::IntoResponse;
use puzz_core::service::util::BoxService;
use puzz_core::service::{Service, ServiceExt};
use puzz_core::{BoxError, Request, Response};

use crate::error::MethodNotAllowed;
use crate::RouteFuture;

#[derive(Debug)]
pub struct MethodRouter {
    options: Option<BoxService<Request, Response, BoxError>>,
    get: Option<BoxService<Request, Response, BoxError>>,
    post: Option<BoxService<Request, Response, BoxError>>,
    put: Option<BoxService<Request, Response, BoxError>>,
    delete: Option<BoxService<Request, Response, BoxError>>,
    head: Option<BoxService<Request, Response, BoxError>>,
    trace: Option<BoxService<Request, Response, BoxError>>,
    patch: Option<BoxService<Request, Response, BoxError>>,
}

macro_rules! router_impl_method_fn {
    ($method:ident) => {
        pub fn $method<S>(mut self, service: S) -> Self
        where
            S: Service<Request> + 'static,
            S::Response: IntoResponse,
            S::Error: Into<BoxError>,
        {
            self.$method = Some(Self::into_box_service(service));
            self
        }
    };
}

impl MethodRouter {
    fn new() -> Self {
        Self {
            options: None,
            get: None,
            post: None,
            put: None,
            delete: None,
            head: None,
            trace: None,
            patch: None,
        }
    }

    router_impl_method_fn!(options);
    router_impl_method_fn!(get);
    router_impl_method_fn!(post);
    router_impl_method_fn!(put);
    router_impl_method_fn!(delete);
    router_impl_method_fn!(head);
    router_impl_method_fn!(trace);
    router_impl_method_fn!(patch);

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

impl Service<Request> for MethodRouter {
    type Response = Response;
    type Error = BoxError;
    type Future = RouteFuture;

    fn call(&self, request: Request) -> Self::Future {
        macro_rules! call {
            ($req:expr, $method:expr, $svc:expr) => {
                if $method == $req.method() {
                    if let Some(svc) = $svc {
                        return RouteFuture::Future {
                            fut: svc.call($req),
                        };
                    }
                }
            };
        }

        call!(request, Method::HEAD, &self.head);
        call!(request, Method::HEAD, &self.get);
        call!(request, Method::GET, &self.get);
        call!(request, Method::POST, &self.post);
        call!(request, Method::OPTIONS, &self.options);
        call!(request, Method::PATCH, &self.patch);
        call!(request, Method::PUT, &self.put);
        call!(request, Method::DELETE, &self.delete);
        call!(request, Method::TRACE, &self.trace);

        RouteFuture::Error {
            err: Some(MethodNotAllowed::new(request).into()),
        }
    }
}

macro_rules! impl_method_fn {
    ($method:ident) => {
        pub fn $method<S>(service: S) -> MethodRouter
        where
            S: Service<Request> + 'static,
            S::Response: IntoResponse,
            S::Error: Into<BoxError>,
        {
            MethodRouter::new().$method(service)
        }
    };
}

impl_method_fn!(options);
impl_method_fn!(get);
impl_method_fn!(post);
impl_method_fn!(put);
impl_method_fn!(delete);
impl_method_fn!(head);
impl_method_fn!(trace);
impl_method_fn!(patch);

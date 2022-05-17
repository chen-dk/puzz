#![no_std]
#![forbid(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

use core::future::Future;

#[macro_use]
mod macros;

#[cfg(feature = "util")]
pub mod util;
#[cfg(feature = "util")]
pub use util::ServiceExt;

/// 请求响应模型，表示一个接收请求并返回响应的异步函数。
pub trait Service<Request> {
    /// 服务返回的响应。
    type Response;

    /// 服务产生的错误。
    type Error;

    /// 异步返回的响应。
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    /// 处理请求并异步返回响应。
    fn call(&self, request: Request) -> Self::Future;
}

impl<'a, S, Request> Service<Request> for &'a mut S
where
    S: Service<Request> + ?Sized + 'a,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn call(&self, request: Request) -> Self::Future {
        (**self).call(request)
    }
}

impl<'a, S, Request> Service<Request> for &'a S
where
    S: Service<Request> + ?Sized + 'a,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn call(&self, request: Request) -> Self::Future {
        (**self).call(request)
    }
}

#[cfg(feature = "alloc")]
impl<S, Request> Service<Request> for alloc::boxed::Box<S>
where
    S: Service<Request> + ?Sized,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn call(&self, request: Request) -> Self::Future {
        (**self).call(request)
    }
}

#[cfg(feature = "alloc")]
impl<S, Request> Service<Request> for alloc::rc::Rc<S>
where
    S: Service<Request> + ?Sized,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn call(&self, request: Request) -> Self::Future {
        (**self).call(request)
    }
}

/// 包裹服务，转换服务的请求和响应。
pub trait Wrap<S> {
    /// 包裹后的服务。
    type Service;

    /// 包裹给定的服务，返回一个包裹后的新服务。
    fn wrap(self, service: S) -> Self::Service;
}

use core::{fmt, future::Future, pin::Pin};

use alloc::{boxed::Box, rc::Rc};

use crate::{Service, ServiceExt};

/// 装箱的[`Future`]特征对象。
///
/// 此类型别名表示一个装箱的[`Future`]，不可以跨线程移动。
pub type BoxFuture<T> = Pin<Box<dyn Future<Output = T>>>;

/// 装箱的[`Service`]特征对象。
///
/// [`BoxService`]将服务转换为特征对象并装箱，允许[`Service::Future`]是动态的。
///
/// 如果需要一个实现[`Clone`]的装箱服务，考虑使用[`BoxCloneService`]或[`RcService`]。
pub struct BoxService<Req, Res, Err> {
    inner: Box<dyn Service<Req, Response = Res, Error = Err, Future = BoxFuture<Result<Res, Err>>>>,
}

impl<Req, Res, Err> BoxService<Req, Res, Err> {
    pub fn new<S>(inner: S) -> Self
    where
        S: Service<Req, Response = Res, Error = Err> + 'static,
        S::Future: 'static,
    {
        Self {
            inner: Box::new(inner.map_future(|f| Box::pin(f) as _)),
        }
    }
}

impl<Req, Res, Err> Service<Req> for BoxService<Req, Res, Err> {
    type Response = Res;
    type Error = Err;
    type Future = BoxFuture<Result<Res, Err>>;

    fn call(&self, request: Req) -> BoxFuture<Result<Res, Err>> {
        self.inner.call(request)
    }
}

impl<Req, Res, Err> fmt::Debug for BoxService<Req, Res, Err> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("BoxService").finish()
    }
}

/// 装箱的[`Service`]特征对象。
///
/// [`BoxCloneService`]将服务转换为特征对象并装箱，允许[`Service::Future`]是动态的，
/// 并允许克隆服务。
///
/// 这与[`BoxService`]类似，只是[`BoxCloneService`]实现了[`Clone`]。
pub struct BoxCloneService<Req, Res, Err> {
    inner: Box<
        dyn CloneService<Req, Response = Res, Error = Err, Future = BoxFuture<Result<Res, Err>>>,
    >,
}

impl<Req, Res, Err> Clone for BoxCloneService<Req, Res, Err> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone_box(),
        }
    }
}

impl<Req, Res, Err> BoxCloneService<Req, Res, Err> {
    pub fn new<S>(inner: S) -> Self
    where
        S: Service<Req, Response = Res, Error = Err> + Clone + 'static,
        S::Future: 'static,
    {
        BoxCloneService {
            inner: Box::new(inner.map_future(|f| Box::pin(f) as _)),
        }
    }
}

impl<Req, Res, Err> Service<Req> for BoxCloneService<Req, Res, Err> {
    type Response = Res;
    type Error = Err;
    type Future = BoxFuture<Result<Res, Err>>;

    fn call(&self, request: Req) -> BoxFuture<Result<Res, Err>> {
        self.inner.call(request)
    }
}

impl<Req, Res, Err> fmt::Debug for BoxCloneService<Req, Res, Err> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("BoxCloneService").finish()
    }
}

/// 装箱的[`Service`]特征对象。
///
/// [`RcService`]将服务转换为特征对象并装箱，允许[`Service::Future`]是动态的，
/// 并允许共享服务。
///
/// 这与[`BoxService`]类似，只是[`RcService`]实现了[`Clone`]。
pub struct RcService<Req, Res, Err> {
    inner: Rc<dyn Service<Req, Response = Res, Error = Err, Future = BoxFuture<Result<Res, Err>>>>,
}

impl<Req, Res, Err> Clone for RcService<Req, Res, Err> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<Req, Res, Err> RcService<Req, Res, Err> {
    pub fn new<S>(inner: S) -> Self
    where
        S: Service<Req, Response = Res, Error = Err> + 'static,
        S::Future: 'static,
    {
        Self {
            inner: Rc::new(inner.map_future(|f| Box::pin(f) as _)),
        }
    }
}

impl<Req, Res, Err> Service<Req> for RcService<Req, Res, Err> {
    type Response = Res;
    type Error = Err;
    type Future = BoxFuture<Result<Res, Err>>;

    fn call(&self, request: Req) -> BoxFuture<Result<Res, Err>> {
        self.inner.call(request)
    }
}

impl<Req, Res, Err> fmt::Debug for RcService<Req, Res, Err> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("RcService").finish()
    }
}

trait CloneService<R>: Service<R> {
    fn clone_box(
        &self,
    ) -> Box<
        dyn CloneService<R, Response = Self::Response, Error = Self::Error, Future = Self::Future>,
    >;
}

impl<S, R> CloneService<R> for S
where
    S: Service<R> + Clone + 'static,
{
    fn clone_box(
        &self,
    ) -> Box<
        dyn CloneService<R, Response = Self::Response, Error = Self::Error, Future = Self::Future>,
    > {
        Box::new(self.clone())
    }
}

use core::{fmt, future::Future};

use crate::Service;

#[derive(Clone, Copy)]
pub struct MapFuture<S, F> {
    inner: S,
    f: F,
}

impl<S, F> MapFuture<S, F> {
    pub fn new(inner: S, f: F) -> Self {
        Self { inner, f }
    }
}

impl<S, F, Req, Res, Err, Fut> Service<Req> for MapFuture<S, F>
where
    S: Service<Req>,
    F: Fn(S::Future) -> Fut,
    Fut: Future<Output = Result<Res, Err>>,
{
    type Response = Res;
    type Error = Err;
    type Future = Fut;

    fn call(&self, request: Req) -> Self::Future {
        (self.f)(self.inner.call(request))
    }
}

impl<S, F> fmt::Debug for MapFuture<S, F>
where
    S: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MapFuture")
            .field("inner", &self.inner)
            .field("f", &core::any::type_name::<F>())
            .finish()
    }
}

use core::fmt;

use futures_util::FutureExt;

use crate::Service;

opaque_future! {
    pub type MapResultFuture<Fut, F> = futures_util::future::Map<Fut, F>;
}

#[derive(Clone, Copy)]
pub struct MapResult<S, F> {
    inner: S,
    f: F,
}

impl<S, F> MapResult<S, F> {
    pub fn new(inner: S, f: F) -> Self {
        Self { inner, f }
    }
}

impl<S, F, Req, Res, Err> Service<Req> for MapResult<S, F>
where
    S: Service<Req>,
    F: FnOnce(Result<S::Response, S::Error>) -> Result<Res, Err> + Clone,
{
    type Response = Res;
    type Error = Err;
    type Future = MapResultFuture<S::Future, F>;

    fn call(&self, request: Req) -> Self::Future {
        MapResultFuture::new(self.inner.call(request).map(self.f.clone()))
    }
}

impl<S, F> fmt::Debug for MapResult<S, F>
where
    S: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MapResult")
            .field("inner", &self.inner)
            .field("f", &core::any::type_name::<F>())
            .finish()
    }
}

use core::fmt;

use futures_util::TryFutureExt;

use crate::Service;

opaque_future! {
    pub type MapResponseFuture<Fut, F> = futures_util::future::MapOk<Fut, F>;
}

#[derive(Clone, Copy)]
pub struct MapResponse<S, F> {
    inner: S,
    f: F,
}

impl<S, F> MapResponse<S, F> {
    pub fn new(inner: S, f: F) -> Self {
        Self { inner, f }
    }
}

impl<S, F, Req, Res> Service<Req> for MapResponse<S, F>
where
    S: Service<Req>,
    F: FnOnce(S::Response) -> Res + Clone,
{
    type Response = Res;
    type Error = S::Error;
    type Future = MapResponseFuture<S::Future, F>;

    fn call(&self, request: Req) -> Self::Future {
        MapResponseFuture::new(self.inner.call(request).map_ok(self.f.clone()))
    }
}

impl<S, F> fmt::Debug for MapResponse<S, F>
where
    S: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MapResponse")
            .field("inner", &self.inner)
            .field("f", &core::any::type_name::<F>())
            .finish()
    }
}

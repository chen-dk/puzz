use core::{fmt, future::Future};

use futures_util::FutureExt;

use crate::Service;

opaque_future! {
    pub type ThenFuture<Fut1, Fut2, F> = futures_util::future::Then<Fut1, Fut2, F>;
}

#[derive(Clone, Copy)]
pub struct Then<S, F> {
    inner: S,
    f: F,
}

impl<S, F> Then<S, F> {
    pub fn new(inner: S, f: F) -> Self {
        Self { inner, f }
    }
}

impl<S, F, Req, Res, Err, Fut> Service<Req> for Then<S, F>
where
    S: Service<Req>,
    F: FnOnce(Result<S::Response, S::Error>) -> Fut + Clone,
    Fut: Future<Output = Result<Res, Err>>,
{
    type Response = Res;
    type Error = Err;
    type Future = ThenFuture<S::Future, Fut, F>;

    fn call(&self, request: Req) -> Self::Future {
        ThenFuture::new(self.inner.call(request).then(self.f.clone()))
    }
}

impl<S, F> fmt::Debug for Then<S, F>
where
    S: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Then")
            .field("inner", &self.inner)
            .field("f", &core::any::type_name::<F>())
            .finish()
    }
}

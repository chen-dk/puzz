use std::fmt;

use puzz_core::service::{Service, Wrap};
use puzz_core::Request;

pub fn add_extension<F>(f: F) -> AddExtensionWrap<F> {
    AddExtensionWrap::new(f)
}

#[derive(Clone, Copy)]
pub struct AddExtensionWrap<F> {
    f: F,
}

impl<F> AddExtensionWrap<F> {
    pub fn new(f: F) -> Self {
        Self { f }
    }
}

impl<S, F> Wrap<S> for AddExtensionWrap<F> {
    type Service = AddExtension<S, F>;

    fn wrap(self, service: S) -> Self::Service {
        AddExtension {
            inner: service,
            f: self.f,
        }
    }
}

impl<F> fmt::Debug for AddExtensionWrap<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AddExtensionWrap")
            .field("f", &core::any::type_name::<F>())
            .finish()
    }
}

#[derive(Clone, Copy)]
pub struct AddExtension<S, F> {
    inner: S,
    f: F,
}

impl<S, F, B, T> Service<Request<B>> for AddExtension<S, F>
where
    S: Service<Request<B>>,
    F: Fn() -> T,
    T: 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn call(&self, mut request: Request<B>) -> Self::Future {
        request.extensions_mut().insert((self.f)());
        self.inner.call(request)
    }
}

impl<S, F> fmt::Debug for AddExtension<S, F>
where
    S: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AddExtension")
            .field("inner", &self.inner)
            .field("f", &core::any::type_name::<F>())
            .finish()
    }
}

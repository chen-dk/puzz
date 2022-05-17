use core::{fmt, future::Future};

use crate::Service;

/// 使用异步函数构建一个[`Service`]。
///
/// # 例子
///
/// ```
/// use puzz_service::util::service_fn;
///
/// let service = service_fn(|_: ()| async {
///     Ok::<_, ()>("Hello World")
/// });
/// ```
pub fn service_fn<F>(f: F) -> ServiceFn<F> {
    ServiceFn::new(f)
}

#[derive(Clone, Copy)]
pub struct ServiceFn<F> {
    f: F,
}

impl<F> ServiceFn<F> {
    pub(crate) fn new(f: F) -> Self {
        Self { f }
    }
}

impl<F, Req, Res, Err, Fut> Service<Req> for ServiceFn<F>
where
    F: Fn(Req) -> Fut,
    Fut: Future<Output = Result<Res, Err>>,
{
    type Response = Res;
    type Error = Err;
    type Future = Fut;

    fn call(&self, request: Req) -> Self::Future {
        (self.f)(request)
    }
}

impl<F> fmt::Debug for ServiceFn<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ServiceFn")
            .field("f", &core::any::type_name::<F>())
            .finish()
    }
}

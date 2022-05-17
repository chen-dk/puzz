use core::fmt;

use crate::Wrap;

/// 使用函数构建一个[`Wrap`]。
///
/// # 例子
///
/// ```
/// use puzz_service::{Service, ServiceExt};
/// use puzz_service::util::{service_fn, wrap_fn, BoxService};
///
/// let wrap = wrap_fn(|servcie: BoxService<(), (), ()>| {
///     servcie.map_response(|_: ()| "Hello World")
/// });
///
/// let service = service_fn(|_: ()| async {
///     Ok::<_, ()>(())
/// }).boxed();
///
/// let service = service.with(wrap);
/// ```
pub fn wrap_fn<F>(f: F) -> WrapFn<F> {
    WrapFn::new(f)
}

#[derive(Clone, Copy)]
pub struct WrapFn<F> {
    f: F,
}

impl<F> WrapFn<F> {
    pub(crate) fn new(f: F) -> Self {
        Self { f }
    }
}

impl<F, S1, S2> Wrap<S1> for WrapFn<F>
where
    F: FnOnce(S1) -> S2,
{
    type Service = S2;

    fn wrap(self, service: S1) -> Self::Service {
        (self.f)(service)
    }
}

impl<F> fmt::Debug for WrapFn<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WrapFn")
            .field("f", &core::any::type_name::<F>())
            .finish()
    }
}

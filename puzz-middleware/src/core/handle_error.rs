use std::convert::Infallible;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_core::ready;
use pin_project_lite::pin_project;
use puzz_core::service::{Service, Wrap};

pub fn handle_error<F>(f: F) -> HandleErrorWrap<F> {
    HandleErrorWrap::new(f)
}

#[derive(Clone, Copy)]
pub struct HandleErrorWrap<F> {
    f: F,
}

impl<F> HandleErrorWrap<F> {
    pub fn new(f: F) -> Self {
        Self { f }
    }
}

impl<S, F> Wrap<S> for HandleErrorWrap<F> {
    type Service = HandleError<S, F>;

    fn wrap(self, service: S) -> Self::Service {
        HandleError {
            inner: service,
            f: self.f,
        }
    }
}

impl<F> fmt::Debug for HandleErrorWrap<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HandleErrorWrap")
            .field("f", &core::any::type_name::<F>())
            .finish()
    }
}

#[derive(Clone, Copy)]
pub struct HandleError<S, F> {
    inner: S,
    f: F,
}

impl<S, F, Req> Service<Req> for HandleError<S, F>
where
    S: Service<Req>,
    F: FnOnce(S::Error) -> S::Response + Clone,
{
    type Response = S::Response;
    type Error = Infallible;
    type Future = HandleErrorFuture<S::Future, F>;

    fn call(&self, request: Req) -> Self::Future {
        HandleErrorFuture::Incomplete {
            fut: self.inner.call(request),
            f: self.f.clone(),
        }
    }
}

impl<S, F> fmt::Debug for HandleError<S, F>
where
    S: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HandleError")
            .field("inner", &self.inner)
            .field("f", &core::any::type_name::<F>())
            .finish()
    }
}

pin_project! {
    #[project = HandleErrorFutureProj]
    #[project_replace = HandleErrorFutureProjReplace]
    pub enum HandleErrorFuture<Fut, F> {
        Incomplete {
            #[pin]
            fut: Fut,
            f: F,
        },
        Complete,
    }
}

impl<Fut, F, Res, Err> Future for HandleErrorFuture<Fut, F>
where
    Fut: Future<Output = Result<Res, Err>>,
    F: FnOnce(Err) -> Res,
{
    type Output = Result<Res, Infallible>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.as_mut().project() {
            HandleErrorFutureProj::Incomplete { fut, .. } => {
                let output = ready!(fut.poll(cx));
                match self.project_replace(HandleErrorFuture::Complete) {
                    HandleErrorFutureProjReplace::Incomplete { f, .. } => match output {
                        Ok(res) => Poll::Ready(Ok(res)),
                        Err(err) => Poll::Ready(Ok(f(err))),
                    },
                    HandleErrorFutureProjReplace::Complete => unreachable!(),
                }
            }
            HandleErrorFutureProj::Complete => {
                panic!("polled after completion")
            }
        }
    }
}

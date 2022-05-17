#![forbid(unsafe_code)]

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use pin_project_lite::pin_project;
use puzz_core::service::util::BoxFuture;
use puzz_core::{BoxError, Response};

mod method;
mod router;

pub mod error;

pub use method::*;
pub use router::*;

pin_project! {
    #[project = RouteFutureProj]
    pub enum RouteFuture {
        Future {
            #[pin]
            fut: BoxFuture<Result<Response, BoxError>>,
        },
        Error {
            err: Option<BoxError>,
        },
    }
}

impl Future for RouteFuture {
    type Output = Result<Response, BoxError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.project() {
            RouteFutureProj::Future { fut } => fut.poll(cx),
            RouteFutureProj::Error { err } => {
                Poll::Ready(Err(err.take().expect("polled after completion").into()))
            }
        }
    }
}

use std::future::Future;
use std::pin::Pin;
use std::task;

use bytes::Bytes;

use super::Body;

#[must_use = "futures don't do anything unless polled"]
#[derive(Debug)]
pub struct Next<'a, B: ?Sized>(pub(crate) &'a mut B);

impl<'a, B: Body + Unpin + ?Sized> Future for Next<'a, B> {
    type Output = Option<Result<Bytes, B::Error>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        Pin::new(&mut self.0).poll_next(cx)
    }
}

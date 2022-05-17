use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::Bytes;
use futures_core::{Stream, TryStream};
use pin_project_lite::pin_project;

use super::Body;

pin_project! {
    #[derive(Debug, Default)]
    pub struct StreamBody<S> {
        #[pin]
        stream: S,
    }
}

impl<S> StreamBody<S> {
    pub fn new(stream: S) -> Self {
        Self { stream }
    }
}

impl<S> Body for StreamBody<S>
where
    S: TryStream,
    S::Ok: Into<Bytes>,
{
    type Error = S::Error;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        match self.project().stream.try_poll_next(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Ready(Some(Ok(data))) => Poll::Ready(Some(Ok(data.into()))),
            Poll::Ready(Some(Err(err))) => Poll::Ready(Some(Err(err))),
        }
    }
}

pin_project! {
    #[derive(Debug, Default)]
    pub struct BodyStream<B> {
        #[pin]
        body: B,
    }
}

impl<B> BodyStream<B> {
    pub fn new(body: B) -> Self {
        Self { body }
    }
}

impl<B> Stream for BodyStream<B>
where
    B: Body,
{
    type Item = Result<Bytes, B::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.project().body.poll_next(cx)
    }
}

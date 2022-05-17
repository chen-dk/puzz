use std::fmt;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::Bytes;
use pin_project_lite::pin_project;

use super::{Body, SizeHint};

pin_project! {
    #[derive(Clone, Copy)]
    pub struct MapErr<B, F> {
        #[pin]
        inner: B,
        f: F
    }
}

impl<B, F> MapErr<B, F> {
    pub(crate) fn new(body: B, f: F) -> Self {
        Self { inner: body, f }
    }

    pub fn get_ref(&self) -> &B {
        &self.inner
    }

    pub fn get_mut(&mut self) -> &mut B {
        &mut self.inner
    }

    pub fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut B> {
        self.project().inner
    }

    pub fn into_inner(self) -> B {
        self.inner
    }
}

impl<B, F, E> Body for MapErr<B, F>
where
    B: Body,
    F: FnMut(B::Error) -> E,
{
    type Error = E;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        let this = self.project();
        match this.inner.poll_next(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Ready(Some(Ok(data))) => Poll::Ready(Some(Ok(data))),
            Poll::Ready(Some(Err(err))) => Poll::Ready(Some(Err((this.f)(err)))),
        }
    }

    fn size_hint(&self) -> SizeHint {
        self.inner.size_hint()
    }
}

impl<B, F> fmt::Debug for MapErr<B, F>
where
    B: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MapErr")
            .field("inner", &self.inner)
            .field("f", &std::any::type_name::<F>())
            .finish()
    }
}

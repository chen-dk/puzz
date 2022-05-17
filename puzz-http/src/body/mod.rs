use std::borrow::Cow;
use std::convert::Infallible;
use std::ops::DerefMut;
use std::pin::Pin;
use std::task::{Context, Poll};

pub use bytes::Bytes;

use crate::{Request, Response};

mod boxed;
pub use boxed::BoxBody;

mod ext;
pub use ext::BodyExt;

mod map_err;
pub use map_err::MapErr;

mod next;
pub use next::Next;

mod stream;
pub use stream::{BodyStream, StreamBody};

mod size_hint;
pub use size_hint::SizeHint;

/// 请求或响应的正文特征。
pub trait Body {
    /// 正文产生的错误。
    type Error;

    /// 尝试提取正文的下一个数据，如果正文耗尽则返回[`None`]。
    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>>;

    /// 返回正文剩余长度的界限。
    fn size_hint(&self) -> SizeHint {
        SizeHint::default()
    }
}

impl<B> Body for &mut B
where
    B: Body + Unpin + ?Sized,
{
    type Error = B::Error;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        Pin::new(&mut **self).poll_next(cx)
    }

    fn size_hint(&self) -> SizeHint {
        (**self).size_hint()
    }
}

impl<P> Body for Pin<P>
where
    P: DerefMut + Unpin,
    P::Target: Body,
{
    type Error = <P::Target as Body>::Error;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        self.get_mut().as_mut().poll_next(cx)
    }

    fn size_hint(&self) -> SizeHint {
        (**self).size_hint()
    }
}

impl<B> Body for Box<B>
where
    B: Body + Unpin + ?Sized,
{
    type Error = B::Error;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        Pin::new(&mut **self).poll_next(cx)
    }

    fn size_hint(&self) -> SizeHint {
        (**self).size_hint()
    }
}

impl<B> Body for Request<B>
where
    B: Body,
{
    type Error = B::Error;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        unsafe { self.map_unchecked_mut(Request::body_mut).poll_next(cx) }
    }

    fn size_hint(&self) -> SizeHint {
        self.body().size_hint()
    }
}

impl<B> Body for Response<B>
where
    B: Body,
{
    type Error = B::Error;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        unsafe { self.map_unchecked_mut(Response::body_mut).poll_next(cx) }
    }

    fn size_hint(&self) -> SizeHint {
        self.body().size_hint()
    }
}

impl Body for () {
    type Error = Infallible;

    fn poll_next(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        Poll::Ready(None)
    }

    fn size_hint(&self) -> SizeHint {
        SizeHint::with_exact(0)
    }
}

impl Body for &'static [u8] {
    type Error = Infallible;

    fn poll_next(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        if self.is_empty() {
            Poll::Ready(None)
        } else {
            Poll::Ready(Some(Ok(Bytes::from_static(std::mem::take(self.get_mut())))))
        }
    }

    fn size_hint(&self) -> SizeHint {
        SizeHint::with_exact(self.len() as u64)
    }
}

impl Body for Vec<u8> {
    type Error = Infallible;

    fn poll_next(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        if self.is_empty() {
            Poll::Ready(None)
        } else {
            Poll::Ready(Some(Ok(Bytes::from(std::mem::take(self.get_mut())))))
        }
    }

    fn size_hint(&self) -> SizeHint {
        SizeHint::with_exact(self.len() as u64)
    }
}

impl Body for Cow<'static, [u8]> {
    type Error = Infallible;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        match self.get_mut() {
            Cow::Borrowed(v) => Pin::new(v).poll_next(cx),
            Cow::Owned(v) => Pin::new(v).poll_next(cx),
        }
    }

    fn size_hint(&self) -> SizeHint {
        SizeHint::with_exact(self.len() as u64)
    }
}

impl Body for &'static str {
    type Error = Infallible;

    fn poll_next(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        if self.is_empty() {
            Poll::Ready(None)
        } else {
            Poll::Ready(Some(Ok(Bytes::from_static(
                std::mem::take(self.get_mut()).as_bytes(),
            ))))
        }
    }

    fn size_hint(&self) -> SizeHint {
        SizeHint::with_exact(self.len() as u64)
    }
}

impl Body for String {
    type Error = Infallible;

    fn poll_next(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        if self.is_empty() {
            Poll::Ready(None)
        } else {
            Poll::Ready(Some(Ok(Bytes::from(std::mem::take(self.get_mut())))))
        }
    }

    fn size_hint(&self) -> SizeHint {
        SizeHint::with_exact(self.len() as u64)
    }
}

impl Body for Cow<'static, str> {
    type Error = Infallible;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        match self.get_mut() {
            Cow::Borrowed(v) => Pin::new(v).poll_next(cx),
            Cow::Owned(v) => Pin::new(v).poll_next(cx),
        }
    }

    fn size_hint(&self) -> SizeHint {
        SizeHint::with_exact(self.len() as u64)
    }
}

impl Body for Bytes {
    type Error = Infallible;

    fn poll_next(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        if self.is_empty() {
            Poll::Ready(None)
        } else {
            Poll::Ready(Some(Ok(std::mem::take(self.get_mut()))))
        }
    }

    fn size_hint(&self) -> SizeHint {
        SizeHint::with_exact(self.len() as u64)
    }
}

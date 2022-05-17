use std::borrow::Cow;

use puzz_http::body::{Body, BodyExt, BoxBody, Bytes, MapErr, StreamBody};
use puzz_http::{header, HeaderMap, HeaderValue, StatusCode};

use crate::BoxError;

pub type Response<B = BoxBody> = puzz_http::Response<B>;

pub trait IntoResponse {
    fn into_response(self) -> Response;
}

impl IntoResponse for () {
    fn into_response(self) -> Response {
        Response::new(self.boxed())
    }
}

impl IntoResponse for &'static str {
    fn into_response(self) -> Response {
        Cow::Borrowed(self).into_response()
    }
}

impl IntoResponse for String {
    fn into_response(self) -> Response {
        Cow::<'static, str>::Owned(self).into_response()
    }
}

impl IntoResponse for Cow<'static, str> {
    fn into_response(self) -> Response {
        let mut res = Response::new(self.boxed());
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
        );
        res
    }
}

impl IntoResponse for &'static [u8] {
    fn into_response(self) -> Response {
        Cow::Borrowed(self).into_response()
    }
}

impl IntoResponse for Vec<u8> {
    fn into_response(self) -> Response {
        Cow::<'static, [u8]>::Owned(self).into_response()
    }
}

impl IntoResponse for Cow<'static, [u8]> {
    fn into_response(self) -> Response {
        let mut res = Response::new(self.boxed());
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static(mime::APPLICATION_OCTET_STREAM.as_ref()),
        );
        res
    }
}

impl IntoResponse for Bytes {
    fn into_response(self) -> Response {
        let mut res = Response::new(self.boxed());
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static(mime::APPLICATION_OCTET_STREAM.as_ref()),
        );
        res
    }
}

impl IntoResponse for BoxBody {
    fn into_response(self) -> Response {
        Response::new(self)
    }
}

impl<S> IntoResponse for StreamBody<S>
where
    Self: Body + 'static,
    <Self as Body>::Error: Into<BoxError>,
{
    fn into_response(self) -> Response {
        Response::new(self.boxed())
    }
}

impl<B, F, E> IntoResponse for MapErr<B, F>
where
    B: Body + 'static,
    F: FnMut(B::Error) -> E + 'static,
    E: Into<BoxError>,
{
    fn into_response(self) -> Response {
        Response::new(self.boxed())
    }
}

impl<B> IntoResponse for Response<B>
where
    B: Body + 'static,
    B::Error: Into<BoxError>,
{
    fn into_response(self) -> Response {
        self.map(BodyExt::boxed)
    }
}

impl IntoResponse for puzz_http::response::Head {
    fn into_response(self) -> Response {
        Response::from_head(self, ().boxed())
    }
}

impl IntoResponse for StatusCode {
    fn into_response(self) -> Response {
        let mut res = Response::new(().boxed());
        *res.status_mut() = self;
        res
    }
}

impl IntoResponse for HeaderMap {
    fn into_response(self) -> Response {
        let mut res = Response::new(().boxed());
        *res.headers_mut() = self;
        res
    }
}

impl<T> IntoResponse for (StatusCode, T)
where
    T: IntoResponse,
{
    fn into_response(self) -> Response {
        let mut res = self.1.into_response();
        *res.status_mut() = self.0;
        res
    }
}

impl<T> IntoResponse for (HeaderMap, T)
where
    T: IntoResponse,
{
    fn into_response(self) -> Response {
        let mut res = self.1.into_response();
        res.headers_mut().extend(self.0);
        res
    }
}

impl<T> IntoResponse for (StatusCode, HeaderMap, T)
where
    T: IntoResponse,
{
    fn into_response(self) -> Response {
        let mut res = self.2.into_response();
        *res.status_mut() = self.0;
        res.headers_mut().extend(self.1);
        res
    }
}

#![forbid(unsafe_code)]

use std::fmt;
use std::pin::Pin;
use std::task::{Context, Poll};

use actix_http::error::PayloadError;
use actix_http::header::HeaderMap as ActixHeaderMap;
use bytes::Bytes;
use futures_util::{Stream, TryStreamExt};
use http::{header, HeaderMap};
use pin_project_lite::pin_project;

pin_project! {
    pub struct Multipart {
        #[pin]
        inner: actix_multipart::Multipart,
    }
}

impl Multipart {
    pub fn new<S>(headers: &HeaderMap, stream: S) -> Result<Self, MultipartError>
    where
        S: Stream<Item = Result<Bytes, Box<dyn std::error::Error>>> + 'static,
    {
        Self::boundary(headers)?;

        let content_type = headers.get(&header::CONTENT_TYPE).unwrap().to_owned();

        let mut headers = ActixHeaderMap::with_capacity(1);
        headers.append(header::CONTENT_TYPE, content_type);

        let stream = stream.map_err(|_| PayloadError::Io(std::io::ErrorKind::Other.into()));

        Ok(Self {
            inner: actix_multipart::Multipart::new(&headers, stream),
        })
    }

    pub(crate) fn boundary(headers: &HeaderMap) -> Result<String, MultipartError> {
        let m = headers
            .get(&header::CONTENT_TYPE)
            .ok_or(MultipartError::UnsupportedContentType)?
            .to_str()
            .ok()
            .and_then(|content_type| content_type.parse::<mime::Mime>().ok())
            .ok_or(MultipartError::UnsupportedContentType)?;

        if !(m.type_() == mime::MULTIPART && m.subtype() == mime::FORM_DATA) {
            return Err(MultipartError::UnsupportedContentType);
        }

        m.get_param(mime::BOUNDARY)
            .map(|boundary| boundary.as_str().to_owned())
            .ok_or(MultipartError::UnsupportedContentType)
    }
}

impl Stream for Multipart {
    type Item = Result<Field, MultipartError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.project().inner.poll_next(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Ready(Some(Ok(field))) => Poll::Ready(Some(Ok(Field::from_actix(field)))),
            Poll::Ready(Some(Err(err))) => {
                Poll::Ready(Some(Err(MultipartError::Other(err.into()))))
            }
        }
    }
}

impl fmt::Debug for Multipart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Multipart").finish()
    }
}

pin_project! {
    pub struct Field {
        #[pin]
        inner: actix_multipart::Field,
        headers: HeaderMap,
    }
}

impl Field {
    fn from_actix(field: actix_multipart::Field) -> Self {
        Self {
            headers: field
                .headers()
                .into_iter()
                .map(|(k, v)| (k.to_owned(), v.to_owned()))
                .collect(),
            inner: field,
        }
    }

    pub fn name(&self) -> &str {
        self.inner.name()
    }

    pub fn filename(&self) -> Option<&str> {
        self.inner.content_disposition().get_filename()
    }

    pub fn content_type(&self) -> &mime::Mime {
        self.inner.content_type()
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }
}

impl Stream for Field {
    type Item = Result<Bytes, MultipartError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.project().inner.poll_next(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Ready(Some(Ok(data))) => Poll::Ready(Some(Ok(data))),
            Poll::Ready(Some(Err(err))) => {
                Poll::Ready(Some(Err(MultipartError::Other(err.into()))))
            }
        }
    }
}

impl fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Field").finish()
    }
}

#[derive(Debug)]
pub enum MultipartError {
    UnsupportedContentType,
    Other(Box<dyn std::error::Error>),
}

impl fmt::Display for MultipartError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MultipartError::UnsupportedContentType => f.write_str("unsupported content type"),
            MultipartError::Other(e) => {
                write!(f, "error parsing `multipart/form-data` request ({})", e)
            }
        }
    }
}

impl std::error::Error for MultipartError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MultipartError::UnsupportedContentType => None,
            MultipartError::Other(e) => Some(e.as_ref()),
        }
    }
}

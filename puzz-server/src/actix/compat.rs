use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use actix_http::body::{BodySize, MessageBody};
use actix_http::Payload;
use futures_core::Stream;
use pin_project_lite::pin_project;
use puzz_core::body::{Body, BodyExt, BoxBody, Bytes};
use puzz_core::response::IntoResponse;
use puzz_core::service::{Service, ServiceExt};
use puzz_core::{BoxError, Request};

use crate::PeerAddr;

pin_project! {
    struct IntoPuzzBody {
        #[pin]
        body: Payload,
    }
}

impl Body for IntoPuzzBody {
    type Error = BoxError;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        self.project().body.poll_next(cx).map_err(Into::into)
    }
}

pin_project! {
    pub(crate) struct IntoActixBody {
        #[pin]
        body: BoxBody,
    }
}

impl MessageBody for IntoActixBody {
    type Error = BoxError;

    fn size(&self) -> BodySize {
        if let Some(size) = self.body.size_hint().exact() {
            BodySize::Sized(size)
        } else {
            BodySize::Stream
        }
    }

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        self.project().body.poll_next(cx)
    }
}

pub(crate) fn into_actix_service<S>(
    service: S,
) -> impl Service<
    actix_http::Request,
    Response = actix_http::Response<IntoActixBody>,
    Error = Infallible,
    Future = impl Future<Output = Result<actix_http::Response<IntoActixBody>, Infallible>>,
>
where
    S: Service<Request, Error = Infallible>,
    S::Response: IntoResponse,
{
    service
        .map_request(|request: actix_http::Request| {
            let (head, body) = request.into_parts();

            let mut request = Request::builder()
                .method(&head.method)
                .uri(&head.uri)
                .version(head.version);

            if let Some(peer_addr) = head.peer_addr {
                request = request.extension(PeerAddr(peer_addr));
            }

            for (k, v) in head.headers.iter() {
                request = request.header(k, v);
            }

            request.body(IntoPuzzBody { body }.boxed()).unwrap()
        })
        .map_response(|response: S::Response| {
            let (head, body) = response.into_response().into_head();

            let mut response = actix_http::Response::build(head.status);

            for (k, v) in head.headers.iter() {
                response.append_header((k, v));
            }

            response.message_body(IntoActixBody { body }).unwrap()
        })
}

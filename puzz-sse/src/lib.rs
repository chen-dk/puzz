#![forbid(unsafe_code)]

use std::borrow::Cow;
use std::fmt::{self, Write};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use futures_core::{ready, Stream};
use pin_project_lite::pin_project;
use puzz_core::body::{Body, BodyExt, Bytes};
use puzz_core::http::header;
use puzz_core::response::IntoResponse;
use puzz_core::{BoxError, Response};

pub struct Sse<S> {
    stream: S,
    keep_alive: Option<KeepAlive>,
}

impl<S> Sse<S> {
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            keep_alive: None,
        }
    }

    pub fn keep_alive(mut self, keep_alive: KeepAlive) -> Self {
        self.keep_alive = Some(keep_alive);
        self
    }
}

impl<S> fmt::Debug for Sse<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Sse")
            .field("stream", &format_args!("{}", std::any::type_name::<S>()))
            .field("keep_alive", &self.keep_alive)
            .finish()
    }
}

impl<S, E> IntoResponse for Sse<S>
where
    S: Stream<Item = Result<Event, E>> + Send + 'static,
    E: Into<BoxError>,
{
    fn into_response(self) -> Response {
        let body = SseBody {
            event_stream: self.stream,
            keep_alive: self.keep_alive.map(KeepAliveStream::new),
        };

        Response::builder()
            .header(header::CONTENT_TYPE, mime::TEXT_EVENT_STREAM.as_ref())
            .header(header::CACHE_CONTROL, "no-cache")
            .body(body.boxed())
            .unwrap()
    }
}

pin_project! {
    struct SseBody<S> {
        #[pin]
        event_stream: S,
        #[pin]
        keep_alive: Option<KeepAliveStream>,
    }
}

impl<S, E> Body for SseBody<S>
where
    S: Stream<Item = Result<Event, E>>,
{
    type Error = E;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        let this = self.project();

        match this.event_stream.poll_next(cx) {
            Poll::Pending => {
                if let Some(keep_alive) = this.keep_alive.as_pin_mut() {
                    keep_alive
                        .poll_event(cx)
                        .map(|e| Some(Ok(Bytes::from(e.to_string()))))
                } else {
                    Poll::Pending
                }
            }
            Poll::Ready(Some(Ok(event))) => {
                if let Some(keep_alive) = this.keep_alive.as_pin_mut() {
                    keep_alive.reset();
                }
                Poll::Ready(Some(Ok(Bytes::from(event.to_string()))))
            }
            Poll::Ready(Some(Err(error))) => Poll::Ready(Some(Err(error))),
            Poll::Ready(None) => Poll::Ready(None),
        }
    }
}

#[derive(Debug, Default)]
pub struct Event {
    id: Option<String>,
    data: Option<DataType>,
    event: Option<String>,
    comment: Option<String>,
    retry: Option<Duration>,
}

#[derive(Debug)]
enum DataType {
    Text(String),

    Json(String),
}

impl Event {
    pub fn data<T>(mut self, data: T) -> Event
    where
        T: Into<String>,
    {
        let data = data.into();
        assert_eq!(
            memchr::memchr(b'\r', data.as_bytes()),
            None,
            "SSE data cannot contain carriage returns",
        );
        self.data = Some(DataType::Text(data));
        self
    }

    pub fn json_data<T>(mut self, data: T) -> serde_json::Result<Event>
    where
        T: serde::Serialize,
    {
        self.data = Some(DataType::Json(serde_json::to_string(&data)?));
        Ok(self)
    }

    pub fn comment<T>(mut self, comment: T) -> Event
    where
        T: Into<String>,
    {
        let comment = comment.into();
        assert_eq!(
            memchr::memchr2(b'\r', b'\n', comment.as_bytes()),
            None,
            "SSE comment cannot contain newlines or carriage returns"
        );
        self.comment = Some(comment);
        self
    }

    pub fn event<T>(mut self, event: T) -> Event
    where
        T: Into<String>,
    {
        let event = event.into();
        assert_eq!(
            memchr::memchr2(b'\r', b'\n', event.as_bytes()),
            None,
            "SSE event name cannot contain newlines or carriage returns"
        );
        self.event = Some(event);
        self
    }

    pub fn retry(mut self, duration: Duration) -> Event {
        self.retry = Some(duration);
        self
    }

    pub fn id<T>(mut self, id: T) -> Event
    where
        T: Into<String>,
    {
        let id = id.into();
        assert_eq!(
            memchr::memchr3(b'\r', b'\n', b'\0', id.as_bytes()),
            None,
            "Event ID cannot contain newlines, carriage returns or null characters",
        );
        self.id = Some(id);
        self
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(comment) = &self.comment {
            ":".fmt(f)?;
            comment.fmt(f)?;
            f.write_char('\n')?;
        }

        if let Some(event) = &self.event {
            "event: ".fmt(f)?;
            event.fmt(f)?;
            f.write_char('\n')?;
        }

        match &self.data {
            Some(DataType::Text(data)) => {
                for line in data.split('\n') {
                    "data: ".fmt(f)?;
                    line.fmt(f)?;
                    f.write_char('\n')?;
                }
            }

            Some(DataType::Json(data)) => {
                "data:".fmt(f)?;
                data.fmt(f)?;
                f.write_char('\n')?;
            }
            None => {}
        }

        if let Some(id) = &self.id {
            "id: ".fmt(f)?;
            id.fmt(f)?;
            f.write_char('\n')?;
        }

        if let Some(duration) = &self.retry {
            "retry:".fmt(f)?;

            let secs = duration.as_secs();
            let millis = duration.subsec_millis();

            if secs > 0 {
                // format seconds
                secs.fmt(f)?;

                // pad milliseconds
                if millis < 10 {
                    f.write_str("00")?;
                } else if millis < 100 {
                    f.write_char('0')?;
                }
            }

            // format milliseconds
            millis.fmt(f)?;

            f.write_char('\n')?;
        }

        f.write_char('\n')?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct KeepAlive {
    comment_text: Cow<'static, str>,
    max_interval: Duration,
}

impl KeepAlive {
    pub fn new() -> Self {
        Self {
            comment_text: Cow::Borrowed(""),
            max_interval: Duration::from_secs(15),
        }
    }

    pub fn interval(mut self, time: Duration) -> Self {
        self.max_interval = time;
        self
    }

    pub fn text<I>(mut self, text: I) -> Self
    where
        I: Into<Cow<'static, str>>,
    {
        self.comment_text = text.into();
        self
    }
}

impl Default for KeepAlive {
    fn default() -> Self {
        Self::new()
    }
}

pin_project! {
    #[derive(Debug)]
    pub(crate) struct KeepAliveStream {
        keep_alive: KeepAlive,
        #[pin]
        alive_timer: tokio::time::Sleep,
    }
}

impl KeepAliveStream {
    pub(crate) fn new(keep_alive: KeepAlive) -> Self {
        Self {
            alive_timer: tokio::time::sleep(keep_alive.max_interval),
            keep_alive,
        }
    }

    pub(crate) fn reset(self: Pin<&mut Self>) {
        let this = self.project();
        this.alive_timer
            .reset(tokio::time::Instant::now() + this.keep_alive.max_interval);
    }

    pub(crate) fn poll_event(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Event> {
        let this = self.as_mut().project();

        ready!(this.alive_timer.poll(cx));

        let comment_str = this.keep_alive.comment_text.clone();
        let event = Event::default().comment(comment_str);

        self.reset();

        Poll::Ready(event)
    }
}

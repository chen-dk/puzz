use super::{Body, BodyStream, BoxBody, MapErr, Next};

pub trait BodyExt: Body {
    fn next(&mut self) -> Next<'_, Self>
    where
        Self: Unpin,
    {
        Next(self)
    }

    fn map_err<F, E>(self, f: F) -> MapErr<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Error) -> E,
    {
        MapErr::new(self, f)
    }

    fn stream(self) -> BodyStream<Self>
    where
        Self: Sized,
    {
        BodyStream::new(self)
    }

    fn boxed(self) -> BoxBody
    where
        Self: Sized + 'static,
        Self::Error: Into<Box<dyn std::error::Error>>,
    {
        BoxBody::new(self)
    }
}

impl<B> BodyExt for B where B: Body {}

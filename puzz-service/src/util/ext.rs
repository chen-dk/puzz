use crate::{Service, Wrap};

use super::{
    AndThen, BoxCloneService, BoxService, MapErr, MapFuture, MapRequest, MapResponse, MapResult,
    RcService, Then,
};

pub trait ServiceExt<Req>: Service<Req> {
    fn with<T>(self, wrap: T) -> T::Service
    where
        Self: Sized,
        T: Wrap<Self>,
    {
        wrap.wrap(self)
    }

    fn and_then<F>(self, f: F) -> AndThen<Self, F>
    where
        Self: Sized,
    {
        AndThen::new(self, f)
    }

    fn then<F>(self, f: F) -> Then<Self, F>
    where
        Self: Sized,
    {
        Then::new(self, f)
    }

    fn map_err<F>(self, f: F) -> MapErr<Self, F>
    where
        Self: Sized,
    {
        MapErr::new(self, f)
    }

    fn map_future<F>(self, f: F) -> MapFuture<Self, F>
    where
        Self: Sized,
    {
        MapFuture::new(self, f)
    }

    fn map_request<F>(self, f: F) -> MapRequest<Self, F>
    where
        Self: Sized,
    {
        MapRequest::new(self, f)
    }

    fn map_response<F>(self, f: F) -> MapResponse<Self, F>
    where
        Self: Sized,
    {
        MapResponse::new(self, f)
    }

    fn map_result<F>(self, f: F) -> MapResult<Self, F>
    where
        Self: Sized,
    {
        MapResult::new(self, f)
    }

    fn boxed(self) -> BoxService<Req, Self::Response, Self::Error>
    where
        Self: Sized + 'static,
        Self::Future: 'static,
    {
        BoxService::new(self)
    }

    fn boxed_clone(self) -> BoxCloneService<Req, Self::Response, Self::Error>
    where
        Self: Sized + Clone + 'static,
        Self::Future: 'static,
    {
        BoxCloneService::new(self)
    }

    fn rc_boxed(self) -> RcService<Req, Self::Response, Self::Error>
    where
        Self: Sized + 'static,
        Self::Future: 'static,
    {
        RcService::new(self)
    }
}

impl<S, Req> ServiceExt<Req> for S where S: Service<Req> {}

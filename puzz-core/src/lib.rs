#![forbid(unsafe_code)]

mod request;

pub mod response;

pub use request::Request;
pub use response::Response;

pub mod body {
    pub use puzz_http::body::*;
}

pub mod http {
    pub use puzz_http::*;
}

pub mod service {
    pub use puzz_service::*;
}
pub use service::util::{service_fn, wrap_fn};

pub type BoxError = Box<dyn std::error::Error>;

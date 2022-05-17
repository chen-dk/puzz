#![forbid(unsafe_code)]

pub mod extract;
pub mod response;

pub use puzz_core::*;

pub mod middleware {
    pub use puzz_middleware::core::{add_extension, handle_error};
}

pub mod route {
    pub use puzz_route::*;
}
pub use route::Router;

#[cfg(feature = "server")]
pub mod server {
    pub use puzz_server::*;
}
#[cfg(feature = "server")]
pub use server::Server;

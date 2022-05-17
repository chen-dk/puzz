#![forbid(unsafe_code)]

use std::{fmt, net::SocketAddr};

#[cfg(feature = "actix")]
mod actix;
#[cfg(feature = "actix")]
pub use actix::Server;

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PeerAddr(pub SocketAddr);

impl fmt::Debug for PeerAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <SocketAddr as fmt::Debug>::fmt(&self.0, f)
    }
}

impl fmt::Display for PeerAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <SocketAddr as fmt::Display>::fmt(&self.0, f)
    }
}

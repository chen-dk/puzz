use std::fmt;

use puzz_core::Request;

#[derive(Debug)]
pub struct NotFound {
    request: Request,
}

impl NotFound {
    pub fn new(request: Request) -> Self {
        Self { request }
    }

    pub fn request_ref(&self) -> &Request {
        &self.request
    }

    pub fn request_mut(&mut self) -> &mut Request {
        &mut self.request
    }

    pub fn into_request(self) -> Request {
        self.request
    }
}

impl fmt::Display for NotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Not Found")
    }
}

impl std::error::Error for NotFound {}

#[derive(Debug)]
pub struct MethodNotAllowed {
    request: Request,
}

impl MethodNotAllowed {
    pub fn new(request: Request) -> Self {
        Self { request }
    }

    pub fn request_ref(&self) -> &Request {
        &self.request
    }

    pub fn request_mut(&mut self) -> &mut Request {
        &mut self.request
    }

    pub fn into_request(self) -> Request {
        self.request
    }
}

impl fmt::Display for MethodNotAllowed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Method Not Allowed")
    }
}

impl std::error::Error for MethodNotAllowed {}

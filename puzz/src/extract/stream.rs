use puzz_core::body::{BodyExt, BodyStream, BoxBody};
use puzz_core::Request;

pub fn stream(request: &mut Request) -> BodyStream<BoxBody> {
    std::mem::take(request.body_mut()).stream()
}

use puzz_core::body::StreamBody;

pub fn stream<S>(stream: S) -> StreamBody<S> {
    StreamBody::new(stream)
}

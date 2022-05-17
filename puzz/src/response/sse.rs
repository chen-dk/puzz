pub use puzz_sse::*;

pub fn sse<S>(stream: S) -> Sse<S> {
    Sse::new(stream)
}

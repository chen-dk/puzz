use puzz_core::Request;

pub use puzz_multipart::*;

pub fn multipart(request: &mut Request) -> Result<Multipart, MultipartError> {
    let stream = crate::extract::stream(request);
    Multipart::new(request.headers(), stream)
}

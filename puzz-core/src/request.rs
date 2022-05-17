use puzz_http::body::BoxBody;

pub type Request<B = BoxBody> = puzz_http::Request<B>;

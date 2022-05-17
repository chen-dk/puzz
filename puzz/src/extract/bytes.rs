use bytes::{Buf, BufMut, Bytes};
use puzz_core::body::{Body, BodyExt};
use puzz_core::{BoxError, Request};

pub async fn bytes(request: &mut Request) -> Result<Bytes, BoxError> {
    let mut body = std::mem::take(request.body_mut());

    let mut buf1 = if let Some(buf) = body.next().await {
        buf?
    } else {
        return Ok(Bytes::new());
    };

    let buf2 = if let Some(buf) = body.next().await {
        buf?
    } else {
        return Ok(buf1.copy_to_bytes(buf1.remaining()));
    };

    let cap = buf1.remaining() + buf2.remaining() + body.size_hint().lower() as usize;
    let mut vec = Vec::with_capacity(cap);

    vec.put(buf1);
    vec.put(buf2);

    while let Some(buf) = body.next().await {
        vec.put(buf?);
    }

    Ok(vec.into())
}

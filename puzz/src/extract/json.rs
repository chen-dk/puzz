use std::fmt;

use puzz_core::http::header;
use puzz_core::{BoxError, Request};
use serde::de::DeserializeOwned;

pub async fn json<T>(request: &mut Request) -> Result<T, ExtractJsonError>
where
    T: DeserializeOwned,
{
    if !is_json_content_type(request) {
        return Err(ExtractJsonError::UnsupportedContentType);
    }

    let bytes = crate::extract::bytes(request)
        .await
        .map_err(|e| ExtractJsonError::FailedToReadBody(e.into()))?;

    serde_json::from_slice(&bytes).map_err(ExtractJsonError::FailedToDeserialize)
}

fn is_json_content_type(request: &Request) -> bool {
    let content_type = if let Some(content_type) = request.headers().get(header::CONTENT_TYPE) {
        content_type
    } else {
        return false;
    };

    let content_type = if let Ok(content_type) = content_type.to_str() {
        content_type
    } else {
        return false;
    };

    let mime = if let Ok(mime) = content_type.parse::<mime::Mime>() {
        mime
    } else {
        return false;
    };

    let is_json_content_type = mime.type_() == "application"
        && (mime.subtype() == "json" || mime.suffix().filter(|name| *name == "json").is_some());

    is_json_content_type
}

#[derive(Debug)]
pub enum ExtractJsonError {
    UnsupportedContentType,
    FailedToReadBody(BoxError),
    FailedToDeserialize(serde_json::Error),
}

impl fmt::Display for ExtractJsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExtractJsonError::UnsupportedContentType => f.write_str("unsupported content type"),
            ExtractJsonError::FailedToReadBody(e) => write!(f, "failed to read body ({})", e),
            ExtractJsonError::FailedToDeserialize(e) => {
                write!(f, "failed to deserialize ({})", e)
            }
        }
    }
}

impl std::error::Error for ExtractJsonError {}

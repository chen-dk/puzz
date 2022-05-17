use std::fmt;
use std::str::FromStr;

use puzz_core::http::HeaderName;
use puzz_core::{BoxError, Request};

pub fn header<T>(request: &Request, name: HeaderName) -> Result<T, ExtractHeaderError>
where
    T: FromStr,
    T::Err: Into<BoxError>,
{
    if let Some(value) = request.headers().get(&name) {
        match value.to_str() {
            Ok(s) => s
                .parse::<T>()
                .map_err(|e| ExtractHeaderError::InvalidHeader {
                    name,
                    source: e.into(),
                }),
            Err(e) => Err(ExtractHeaderError::InvalidHeader {
                name,
                source: e.into(),
            }),
        }
    } else {
        Err(ExtractHeaderError::MissingHeader { name })
    }
}

#[derive(Debug)]
pub enum ExtractHeaderError {
    MissingHeader { name: HeaderName },
    InvalidHeader { name: HeaderName, source: BoxError },
}

impl fmt::Display for ExtractHeaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExtractHeaderError::MissingHeader { name } => {
                write!(f, "missing request header `{}`", name)
            }
            ExtractHeaderError::InvalidHeader { name, source: _ } => {
                write!(f, "invalid request header `{}`", name)
            }
        }
    }
}

impl std::error::Error for ExtractHeaderError {}

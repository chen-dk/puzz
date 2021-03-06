use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use puzz_core::{BoxError, Request};
use puzz_route::Params;

pub fn param_raw<'a>(request: &'a Request, name: &str) -> Option<&'a str> {
    params(request).and_then(|params| params.get(name).map(|v| v.as_str()))
}

pub fn param<T>(request: &Request, name: &str) -> Result<T, ExtractParamError>
where
    T: FromStr,
    T::Err: Into<BoxError>,
{
    param_raw(request, name).map_or_else(
        || Err(ExtractParamError::MissingParam { name: name.into() }),
        |param| {
            param
                .parse::<T>()
                .map_err(|e| ExtractParamError::InvalidParam {
                    name: name.into(),
                    source: e.into(),
                })
        },
    )
}

pub fn params(request: &Request) -> Option<&HashMap<String, String>> {
    crate::extract::extension(request).map(|params: &Params| params.get_ref())
}

#[derive(Debug)]
pub enum ExtractParamError {
    MissingParam { name: String },
    InvalidParam { name: String, source: BoxError },
}

impl fmt::Display for ExtractParamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExtractParamError::MissingParam { name } => {
                write!(f, "missing route param `{}`", name)
            }
            ExtractParamError::InvalidParam { name, source: _ } => {
                write!(f, "invalid route param `{}`", name)
            }
        }
    }
}

impl std::error::Error for ExtractParamError {}

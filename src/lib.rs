mod convert;
mod types;

use std::convert::From;
use crate::types::*;
use napi::bindgen_prelude::Buffer;
use napi::Result as NapiResult;
use rs_trafilatura::Options;
#[allow(unused)]
use napi_derive::napi;

#[napi]
pub fn extract(html: String) -> NapiResult<ExtractResultNapi> {
    use rs_trafilatura::extract;
    extract(&html)
        .map(ExtractResultNapi::from)
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}

#[napi]
pub fn extract_with_options(
    html: String,
    options: Option<OptionsNapi>,
) -> NapiResult<ExtractResultNapi> {
    use rs_trafilatura::extract_with_options;

    let opts = options
        .map(Options::from)
        .unwrap_or_default();

    extract_with_options(&html, &opts)
        .map(ExtractResultNapi::from)
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}

#[napi]
pub fn extract_bytes(
    html: Buffer,
) -> NapiResult<ExtractResultNapi> {
    use rs_trafilatura::extract_bytes;

    let html_bytes: Vec<u8> = html.into();
    extract_bytes(&html_bytes)
        .map(ExtractResultNapi::from)
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}

#[napi]
pub fn extract_bytes_with_options(
    html: Buffer,
    options: Option<OptionsNapi>,
) -> NapiResult<ExtractResultNapi> {
    use rs_trafilatura::extract_bytes_with_options;

    let html_bytes: Vec<u8> = html.into();

    let opts = options
        .map(Options::from)
        .unwrap_or_default();

    extract_bytes_with_options(&html_bytes, &opts)
        .map(ExtractResultNapi::from)
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}
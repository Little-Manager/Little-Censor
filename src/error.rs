//! Error Module
use thiserror::Error as this_error;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, this_error)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub enum Error {
    #[error("Word can't be empty")]
    EmptyWord,

    #[error("Must specify arguments")]
    NoArgs,

    #[error("Provided Invalid Regex")]
    InvalidRegex
}

impl From<regex::Error> for super::Error {
    fn from(_: regex::Error) -> Self {
        Self::InvalidRegex
    }
}

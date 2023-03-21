use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ToolErrors {
    #[error("Base64 decode error.")]
    Base64DecodeError(#[from] base64::DecodeError),
    #[error("Convert from bytes into string error.")]
    BytesToStringError(#[from] std::string::FromUtf8Error),
    #[error("Convert from json str into hashmap error.Input `{0}` is invalid")]
    ConvertJsonToHashMapError(String),
    #[error("Input `{0}` is invalid")]
    InvalidInput(String),
}

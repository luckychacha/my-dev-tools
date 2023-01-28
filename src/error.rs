use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ToolErrors {
    #[error("Base64 decode error.")]
    Base64DecodeError(#[from] base64::DecodeError),
    #[error("Convert from bytes into string error.")]
    BytesToStringError(#[from] std::string::FromUtf8Error),
    #[error("Input `{0}` is invalid")]
    InvalidInput(String),
}

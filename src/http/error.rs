use super::method::MethodError;
use std::str::Utf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
  #[error("Invalid Request")]
  InvalidRequest,
  #[error("Invalid Encoding")]
  InvalidEncoding,
  #[error("Invalid Protocol")]
  InvalidProtocol,
  #[error("Invalid Method")]
  InvalidMethod,
}

// MethodError and Utf8Error are discarded: the variants carry no source payload
// because callers match on the variant alone, not the underlying cause.
impl From<MethodError> for ParseError {
  fn from(_: MethodError) -> Self {
    Self::InvalidMethod
  }
}

impl From<Utf8Error> for ParseError {
  fn from(_: Utf8Error) -> Self {
    Self::InvalidEncoding
  }
}

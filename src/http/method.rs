use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
  GET,
  DELETE,
  POST,
  PUT,
  HEAD,
  CONNECT,
  OPTIONS,
  TRACE,
  PATCH,
}

impl FromStr for Method {
  type Err = MethodError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "GET" => Ok(Self::GET),
      "DELETE" => Ok(Self::DELETE),
      "POST" => Ok(Self::POST),
      "PUT" => Ok(Self::PUT),
      "HEAD" => Ok(Self::HEAD),
      "CONNECT" => Ok(Self::CONNECT),
      "OPTIONS" => Ok(Self::OPTIONS),
      "TRACE" => Ok(Self::TRACE),
      "PATCH" => Ok(Self::PATCH),
      _ => Err(MethodError),
    }
  }
}

pub struct MethodError;

impl fmt::Display for MethodError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Invalid or unrecognized HTTP method")
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::str::FromStr;

  #[test]
  fn all_valid_methods_parse_correctly() {
    assert!(matches!(Method::from_str("GET"), Ok(Method::GET)));
    assert!(matches!(Method::from_str("DELETE"), Ok(Method::DELETE)));
    assert!(matches!(Method::from_str("POST"), Ok(Method::POST)));
    assert!(matches!(Method::from_str("PUT"), Ok(Method::PUT)));
    assert!(matches!(Method::from_str("HEAD"), Ok(Method::HEAD)));
    assert!(matches!(Method::from_str("CONNECT"), Ok(Method::CONNECT)));
    assert!(matches!(Method::from_str("OPTIONS"), Ok(Method::OPTIONS)));
    assert!(matches!(Method::from_str("TRACE"), Ok(Method::TRACE)));
    assert!(matches!(Method::from_str("PATCH"), Ok(Method::PATCH)));
  }

  #[test]
  fn unknown_method_returns_error() {
    assert!(Method::from_str("INVALID").is_err());
    assert!(Method::from_str("").is_err());
    // Parsing is case-sensitive
    assert!(Method::from_str("get").is_err());
  }

  #[test]
  fn method_error_displays_message() {
    assert_eq!(
      format!("{}", MethodError),
      "Invalid or unrecognized HTTP method"
    );
  }
}

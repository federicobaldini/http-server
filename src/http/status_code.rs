use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
  Ok = 200,
  Created = 201,
  NoContent = 204,
  MovedPermanently = 301,
  Found = 302,
  NotModified = 304,
  BadRequest = 400,
  Unauthorized = 401,
  Forbidden = 403,
  NotFound = 404,
  MethodNotAllowed = 405,
  InternalServerError = 500,
  NotImplemented = 501,
  ServiceUnavailable = 503,
}

impl StatusCode {
  pub fn reason_phrase(&self) -> &str {
    match self {
      Self::Ok => "Ok",
      Self::Created => "Created",
      Self::NoContent => "No Content",
      Self::MovedPermanently => "Moved Permanently",
      Self::Found => "Found",
      Self::NotModified => "Not Modified",
      Self::BadRequest => "Bad Request",
      Self::Unauthorized => "Unauthorized",
      Self::Forbidden => "Forbidden",
      Self::NotFound => "Not Found",
      Self::MethodNotAllowed => "Method Not Allowed",
      Self::InternalServerError => "Internal Server Error",
      Self::NotImplemented => "Not Implemented",
      Self::ServiceUnavailable => "Service Unavailable",
    }
  }
}

impl Display for StatusCode {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", *self as u16)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn reason_phrases_match_each_status_code() {
    assert_eq!(StatusCode::Ok.reason_phrase(), "Ok");
    assert_eq!(StatusCode::Created.reason_phrase(), "Created");
    assert_eq!(StatusCode::NoContent.reason_phrase(), "No Content");
    assert_eq!(StatusCode::MovedPermanently.reason_phrase(), "Moved Permanently");
    assert_eq!(StatusCode::Found.reason_phrase(), "Found");
    assert_eq!(StatusCode::NotModified.reason_phrase(), "Not Modified");
    assert_eq!(StatusCode::BadRequest.reason_phrase(), "Bad Request");
    assert_eq!(StatusCode::Unauthorized.reason_phrase(), "Unauthorized");
    assert_eq!(StatusCode::Forbidden.reason_phrase(), "Forbidden");
    assert_eq!(StatusCode::NotFound.reason_phrase(), "Not Found");
    assert_eq!(StatusCode::MethodNotAllowed.reason_phrase(), "Method Not Allowed");
    assert_eq!(StatusCode::InternalServerError.reason_phrase(), "Internal Server Error");
    assert_eq!(StatusCode::NotImplemented.reason_phrase(), "Not Implemented");
    assert_eq!(StatusCode::ServiceUnavailable.reason_phrase(), "Service Unavailable");
  }

  #[test]
  fn display_shows_numeric_code() {
    assert_eq!(format!("{}", StatusCode::Ok), "200");
    assert_eq!(format!("{}", StatusCode::Created), "201");
    assert_eq!(format!("{}", StatusCode::NoContent), "204");
    assert_eq!(format!("{}", StatusCode::MovedPermanently), "301");
    assert_eq!(format!("{}", StatusCode::Found), "302");
    assert_eq!(format!("{}", StatusCode::NotModified), "304");
    assert_eq!(format!("{}", StatusCode::BadRequest), "400");
    assert_eq!(format!("{}", StatusCode::Unauthorized), "401");
    assert_eq!(format!("{}", StatusCode::Forbidden), "403");
    assert_eq!(format!("{}", StatusCode::NotFound), "404");
    assert_eq!(format!("{}", StatusCode::MethodNotAllowed), "405");
    assert_eq!(format!("{}", StatusCode::InternalServerError), "500");
    assert_eq!(format!("{}", StatusCode::NotImplemented), "501");
    assert_eq!(format!("{}", StatusCode::ServiceUnavailable), "503");
  }
}

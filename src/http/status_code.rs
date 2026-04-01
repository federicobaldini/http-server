use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
  Ok = 200,
  Created = 201,
  NoContent = 204,
  BadRequest = 400,
  Unauthorized = 401,
  Forbidden = 403,
  NotFound = 404,
}

impl StatusCode {
  pub fn reason_phrase(&self) -> &str {
    match self {
      Self::Ok => "Ok",
      Self::Created => "Created",
      Self::NoContent => "No Content",
      Self::BadRequest => "Bad Request",
      Self::Unauthorized => "Unauthorized",
      Self::Forbidden => "Forbidden",
      Self::NotFound => "Not Found",
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
    assert_eq!(StatusCode::BadRequest.reason_phrase(), "Bad Request");
    assert_eq!(StatusCode::Unauthorized.reason_phrase(), "Unauthorized");
    assert_eq!(StatusCode::Forbidden.reason_phrase(), "Forbidden");
    assert_eq!(StatusCode::NotFound.reason_phrase(), "Not Found");
  }

  #[test]
  fn display_shows_numeric_code() {
    assert_eq!(format!("{}", StatusCode::Ok), "200");
    assert_eq!(format!("{}", StatusCode::Created), "201");
    assert_eq!(format!("{}", StatusCode::NoContent), "204");
    assert_eq!(format!("{}", StatusCode::BadRequest), "400");
    assert_eq!(format!("{}", StatusCode::Unauthorized), "401");
    assert_eq!(format!("{}", StatusCode::Forbidden), "403");
    assert_eq!(format!("{}", StatusCode::NotFound), "404");
  }
}

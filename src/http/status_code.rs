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

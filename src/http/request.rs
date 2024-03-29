use super::method::{Method, MethodError};
use super::Headers;
use super::QueryString;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct Request<'buf> {
  path: &'buf str,
  query_string: Option<QueryString<'buf>>,
  method: Method,
  headers: Headers<'buf>,
  /*
  body: String,
  */
}

impl<'buf> Request<'buf> {
  pub fn path(&self) -> &str {
    &self.path
  }

  pub fn query_string(&self) -> Option<&QueryString> {
    self.query_string.as_ref()
  }

  pub fn method(&self) -> &Method {
    &self.method
  }

  pub fn headers(&self) -> &Headers {
    &self.headers
  }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
  type Error = ParseError;

  fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
    let request: &str = str::from_utf8(buf)?;

    let (method, request): (&str, &str) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
    let (mut path, request): (&str, &str) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
    let (protocol, request): (&str, &str) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

    if protocol != "HTTP/1.1" {
      return Err(ParseError::InvalidProtocol);
    }

    let method: Method = method.parse()?;

    let mut query_string: Option<QueryString> = None;
    if let Some(i) = path.find('?') {
      query_string = Some(QueryString::from(&path[i + 1..]));
      path = &path[..i];
    }

    let headers = Headers::from(request);

    Ok(Self {
      path,
      query_string,
      method,
      headers,
    })
  }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
  for (i, c) in request.chars().enumerate() {
    if c == ' ' {
      return Some((&request[..i], &request[i + 1..]));
    }
    if c == '\r' || c == '\n' {
      return Some((&request[..i], &request[i + 2..]));
    }
  }
  None
}

pub enum ParseError {
  InvalidRequest,
  InvalidEncoding,
  InvalidProtocol,
  InvalidMethod,
}

impl ParseError {
  fn message(&self) -> &str {
    match self {
      Self::InvalidRequest => "Invalid Request",
      Self::InvalidEncoding => "Invalid Encoding",
      Self::InvalidProtocol => "Invalid Protocol",
      Self::InvalidMethod => "Invalid Method",
    }
  }
}

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

impl Display for ParseError {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", self.message())
  }
}

impl Debug for ParseError {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", self.message())
  }
}

impl Error for ParseError {}

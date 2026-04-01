use super::StatusCode;
use std::io::{Result as IoResult, Write};

#[derive(Debug)]
pub struct Response {
  status_code: StatusCode,
  body: Option<String>,
}

impl Response {
  pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
    Self { status_code, body }
  }

  // The send method sends the Response to the provided stream
  pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
    // Converts the body field (string slice) to a String reference
    let body: &str = match &self.body {
      Some(b) => b,
      None => "",
    };

    // Writes the response to the stream in the format "HTTP/1.1 [status_code] [reason_phrase]\r\n\r\n[body]"
    write!(
      stream,
      "HTTP/1.1 {} {}\r\n\r\n{}",
      self.status_code,
      self.status_code.reason_phrase(),
      body
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn send_with_body_writes_correct_format() {
    let response: Response = Response::new(StatusCode::Ok, Some("Hello".to_string()));
    let mut buf: Vec<u8> = Vec::new();
    response.send(&mut buf).unwrap();
    assert_eq!(
      String::from_utf8(buf).unwrap(),
      "HTTP/1.1 200 Ok\r\n\r\nHello"
    );
  }

  #[test]
  fn send_without_body_writes_empty_body() {
    let response: Response = Response::new(StatusCode::NotFound, None);
    let mut buf: Vec<u8> = Vec::new();
    response.send(&mut buf).unwrap();
    assert_eq!(
      String::from_utf8(buf).unwrap(),
      "HTTP/1.1 404 Not Found\r\n\r\n"
    );
  }

  #[test]
  fn send_bad_request_with_body() {
    let response: Response = Response::new(StatusCode::BadRequest, Some("bad".to_string()));
    let mut buf: Vec<u8> = Vec::new();
    response.send(&mut buf).unwrap();
    assert_eq!(
      String::from_utf8(buf).unwrap(),
      "HTTP/1.1 400 Bad Request\r\n\r\nbad"
    );
  }
}

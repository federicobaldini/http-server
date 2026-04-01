use super::StatusCode;
use std::collections::HashMap;
use std::io::{Result as IoResult, Write};

#[derive(Debug)]
pub struct Response {
  status_code: StatusCode,
  headers: HashMap<String, String>,
  body: Option<String>,
}

impl Response {
  pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
    Self {
      status_code,
      headers: HashMap::new(),
      body,
    }
  }

  pub fn set_header(&mut self, key: String, value: String) {
    self.headers.insert(key, value);
  }

  // The send method sends the Response to the provided stream
  pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
    let body: &str = match &self.body {
      Some(b) => b,
      None => "",
    };

    // Write status line
    write!(
      stream,
      "HTTP/1.1 {} {}\r\n",
      self.status_code,
      self.status_code.reason_phrase()
    )?;

    // Write each header on its own line
    for (key, value) in &self.headers {
      write!(stream, "{}: {}\r\n", key, value)?;
    }

    // Write blank line separating headers from body, then the body
    write!(stream, "\r\n{}", body)
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

  #[test]
  fn send_with_single_header_includes_it_in_output() {
    let mut response: Response = Response::new(StatusCode::Ok, Some("Hi".to_string()));
    response.set_header("Content-Type".to_string(), "text/plain".to_string());
    let mut buf: Vec<u8> = Vec::new();
    response.send(&mut buf).unwrap();
    let output: String = String::from_utf8(buf).unwrap();
    assert!(output.starts_with("HTTP/1.1 200 Ok\r\n"));
    assert!(output.contains("Content-Type: text/plain\r\n"));
    assert!(output.ends_with("\r\nHi"));
  }

  #[test]
  fn send_with_multiple_headers_includes_all() {
    let mut response: Response = Response::new(StatusCode::Ok, None);
    response.set_header("Content-Type".to_string(), "text/html".to_string());
    response.set_header("X-Custom".to_string(), "value".to_string());
    let mut buf: Vec<u8> = Vec::new();
    response.send(&mut buf).unwrap();
    let output: String = String::from_utf8(buf).unwrap();
    assert!(output.contains("Content-Type: text/html\r\n"));
    assert!(output.contains("X-Custom: value\r\n"));
  }

  #[test]
  fn set_header_overwrites_existing_key() {
    let mut response: Response = Response::new(StatusCode::Ok, None);
    response.set_header("X-Foo".to_string(), "first".to_string());
    response.set_header("X-Foo".to_string(), "second".to_string());
    let mut buf: Vec<u8> = Vec::new();
    response.send(&mut buf).unwrap();
    let output: String = String::from_utf8(buf).unwrap();
    assert!(output.contains("X-Foo: second\r\n"));
    assert!(!output.contains("X-Foo: first\r\n"));
  }
}

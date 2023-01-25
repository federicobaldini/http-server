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

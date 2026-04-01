use super::StatusCode;
use std::collections::HashMap;
use std::io::{Result as IoResult, Write};

// Returns the MIME Content-Type for the given file path based on its extension.
// Falls back to "application/octet-stream" for unknown extensions.
pub fn content_type_for_path(path: &str) -> &'static str {
  match path.rsplit_once('.') {
    Some((_, "html")) => "text/html; charset=utf-8",
    Some((_, "css")) => "text/css",
    Some((_, "js")) => "application/javascript",
    Some((_, "json")) => "application/json",
    Some((_, "png")) => "image/png",
    Some((_, "jpg")) | Some((_, "jpeg")) => "image/jpeg",
    Some((_, "gif")) => "image/gif",
    Some((_, "svg")) => "image/svg+xml",
    Some((_, "ico")) => "image/x-icon",
    Some((_, "txt")) => "text/plain; charset=utf-8",
    _ => "application/octet-stream",
  }
}

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
  fn content_type_for_known_extensions() {
    assert_eq!(content_type_for_path("index.html"), "text/html; charset=utf-8");
    assert_eq!(content_type_for_path("style.css"), "text/css");
    assert_eq!(content_type_for_path("app.js"), "application/javascript");
    assert_eq!(content_type_for_path("data.json"), "application/json");
    assert_eq!(content_type_for_path("image.png"), "image/png");
    assert_eq!(content_type_for_path("photo.jpg"), "image/jpeg");
    assert_eq!(content_type_for_path("photo.jpeg"), "image/jpeg");
    assert_eq!(content_type_for_path("anim.gif"), "image/gif");
    assert_eq!(content_type_for_path("icon.svg"), "image/svg+xml");
    assert_eq!(content_type_for_path("favicon.ico"), "image/x-icon");
    assert_eq!(content_type_for_path("readme.txt"), "text/plain; charset=utf-8");
  }

  #[test]
  fn content_type_for_unknown_extension_is_octet_stream() {
    assert_eq!(content_type_for_path("archive.zip"), "application/octet-stream");
    assert_eq!(content_type_for_path("binary"), "application/octet-stream");
  }

  #[test]
  fn content_type_uses_last_extension_in_path() {
    assert_eq!(content_type_for_path("/static/main.min.js"), "application/javascript");
    assert_eq!(content_type_for_path("/assets/theme.dark.css"), "text/css");
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

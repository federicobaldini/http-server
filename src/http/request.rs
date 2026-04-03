use super::error::ParseError;
use super::method::Method;
use super::request_body::RequestBody;
use super::Headers;
use super::QueryString;
use std::convert::TryFrom;
use std::str;

#[derive(Debug)]
pub struct Request<'buf> {
  path: &'buf str,
  query_string: Option<QueryString<'buf>>,
  method: Method,
  headers: Headers<'buf>,
  body: Option<RequestBody>,
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

  pub fn body(&self) -> Option<&RequestBody> {
    self.body.as_ref()
  }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
  type Error = ParseError;

  fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
    // Locate the blank line separating headers from body
    let header_end: usize = match buf.windows(4).position(|w| w == b"\r\n\r\n") {
      Some(i) => i,
      // No separator found: surface encoding errors before InvalidRequest
      None => {
        str::from_utf8(buf)?;
        return Err(ParseError::InvalidRequest);
      }
    };

    // Include the \r\n that terminates the last header line (first two bytes of \r\n\r\n)
    // so get_next_word can find the delimiter after the protocol token
    let header_section: &str = str::from_utf8(&buf[..header_end + 2])?;
    let body_bytes: &[u8] = &buf[header_end + 4..];

    let (method, rest): (&str, &str) = get_next_word(header_section).ok_or(ParseError::InvalidRequest)?;
    let (mut path, rest): (&str, &str) = get_next_word(rest).ok_or(ParseError::InvalidRequest)?;
    let (protocol, rest): (&str, &str) = get_next_word(rest).ok_or(ParseError::InvalidRequest)?;

    if protocol != "HTTP/1.1" {
      return Err(ParseError::InvalidProtocol);
    }

    let method: Method = method.parse()?;

    let mut query_string: Option<QueryString> = None;
    if let Some(i) = path.find('?') {
      query_string = Some(QueryString::from(&path[i + 1..]));
      path = &path[..i];
    }

    let headers: Headers = Headers::from(rest);

    // Parse body using Content-Length; no Content-Length means no body
    let content_length: usize = headers
      .get("Content-Length")
      .and_then(|v| v.parse::<usize>().ok())
      .unwrap_or(0);

    let body: Option<RequestBody> = if content_length == 0 || body_bytes.is_empty() {
      None
    } else {
      let body_slice: &[u8] = &body_bytes[..content_length.min(body_bytes.len())];
      let content_type: &str = headers.get("Content-Type").copied().unwrap_or("");
      // Treat well-known text formats as Text; everything else as Binary
      if content_type.starts_with("text/")
        || content_type.starts_with("application/json")
        || content_type.starts_with("application/x-www-form-urlencoded")
      {
        Some(RequestBody::Text(str::from_utf8(body_slice)?.to_string()))
      } else {
        Some(RequestBody::Binary(body_slice.to_vec()))
      }
    };

    Ok(Self {
      path,
      query_string,
      method,
      headers,
      body,
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


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn valid_get_request_is_parsed() {
    let raw: &[u8] = b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
    let req: Request = Request::try_from(raw).unwrap();
    assert!(matches!(req.method(), Method::GET));
    assert_eq!(req.path(), "/");
    assert!(req.query_string().is_none());
  }

  #[test]
  fn request_with_query_string_separates_path_and_params() {
    let raw: &[u8] = b"GET /search?q=rust&page=1 HTTP/1.1\r\n\r\n";
    let req: Request = Request::try_from(raw).unwrap();
    assert_eq!(req.path(), "/search");
    assert!(req.query_string().is_some());
  }

  #[test]
  fn invalid_utf8_returns_invalid_encoding() {
    let raw: &[u8] = &[0xFF, 0xFE];
    assert!(matches!(
      Request::try_from(raw),
      Err(ParseError::InvalidEncoding)
    ));
  }

  #[test]
  fn non_http11_protocol_returns_invalid_protocol() {
    let raw: &[u8] = b"GET / HTTP/1.0\r\n\r\n";
    assert!(matches!(
      Request::try_from(raw),
      Err(ParseError::InvalidProtocol)
    ));
  }

  #[test]
  fn unknown_method_returns_invalid_method() {
    let raw: &[u8] = b"INVALID / HTTP/1.1\r\n\r\n";
    assert!(matches!(
      Request::try_from(raw),
      Err(ParseError::InvalidMethod)
    ));
  }

  #[test]
  fn truncated_request_returns_invalid_request() {
    let raw: &[u8] = b"GET";
    assert!(matches!(
      Request::try_from(raw),
      Err(ParseError::InvalidRequest)
    ));
  }

  #[test]
  fn parse_error_display_messages() {
    assert_eq!(format!("{}", ParseError::InvalidRequest), "Invalid Request");
    assert_eq!(
      format!("{}", ParseError::InvalidEncoding),
      "Invalid Encoding"
    );
    assert_eq!(
      format!("{}", ParseError::InvalidProtocol),
      "Invalid Protocol"
    );
    assert_eq!(format!("{}", ParseError::InvalidMethod), "Invalid Method");
  }

  #[test]
  fn get_next_word_splits_on_space() {
    assert_eq!(get_next_word("hello world"), Some(("hello", "world")));
  }

  #[test]
  fn get_next_word_splits_on_crlf() {
    assert_eq!(
      get_next_word("hello\r\nworld"),
      Some(("hello", "world"))
    );
  }

  #[test]
  fn get_next_word_on_empty_string_returns_none() {
    assert_eq!(get_next_word(""), None);
  }

  #[test]
  fn get_next_word_with_no_delimiter_returns_none() {
    assert_eq!(get_next_word("hello"), None);
  }

  #[test]
  fn post_with_text_body_is_parsed() {
    let raw: &[u8] =
      b"POST /submit HTTP/1.1\r\nContent-Type: text/plain\r\nContent-Length: 5\r\n\r\nhello";
    let req: Request = Request::try_from(raw).unwrap();
    assert!(matches!(req.method(), Method::POST));
    assert_eq!(req.path(), "/submit");
    assert_eq!(req.body(), Some(&RequestBody::Text("hello".to_string())));
  }

  #[test]
  fn post_with_json_body_is_parsed_as_text() {
    let raw: &[u8] =
      b"POST /api HTTP/1.1\r\nContent-Type: application/json\r\nContent-Length: 15\r\n\r\n{\"key\":\"value\"}";
    let req: Request = Request::try_from(raw).unwrap();
    assert_eq!(
      req.body(),
      Some(&RequestBody::Text("{\"key\":\"value\"}".to_string()))
    );
  }

  #[test]
  fn post_with_binary_body_is_parsed() {
    let mut raw: Vec<u8> =
      b"POST /upload HTTP/1.1\r\nContent-Type: application/octet-stream\r\nContent-Length: 3\r\n\r\n"
        .to_vec();
    raw.extend_from_slice(&[0xDE, 0xAD, 0xBE]);
    let req: Request = Request::try_from(raw.as_slice()).unwrap();
    assert_eq!(
      req.body(),
      Some(&RequestBody::Binary(vec![0xDE, 0xAD, 0xBE]))
    );
  }

  #[test]
  fn get_request_without_body_has_none_body() {
    let raw: &[u8] = b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
    let req: Request = Request::try_from(raw).unwrap();
    assert!(req.body().is_none());
  }

  #[test]
  fn post_without_content_length_has_none_body() {
    let raw: &[u8] = b"POST /submit HTTP/1.1\r\nContent-Type: text/plain\r\n\r\nhello";
    let req: Request = Request::try_from(raw).unwrap();
    assert!(req.body().is_none());
  }
}

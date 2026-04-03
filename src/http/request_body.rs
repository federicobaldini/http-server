#[derive(Debug, PartialEq)]
pub enum RequestBody {
  Text(String),
  Binary(Vec<u8>),
}

impl RequestBody {
  // Returns the body as a byte slice regardless of variant
  pub fn as_bytes(&self) -> &[u8] {
    match self {
      RequestBody::Text(s) => s.as_bytes(),
      RequestBody::Binary(b) => b.as_slice(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn text_variant_stores_string() {
    let body: RequestBody = RequestBody::Text("hello".to_string());
    assert_eq!(body, RequestBody::Text("hello".to_string()));
  }

  #[test]
  fn binary_variant_stores_bytes() {
    let body: RequestBody = RequestBody::Binary(vec![0x01, 0x02, 0x03]);
    assert_eq!(body, RequestBody::Binary(vec![0x01, 0x02, 0x03]));
  }

  #[test]
  fn as_bytes_on_text_returns_utf8_bytes() {
    let body: RequestBody = RequestBody::Text("hi".to_string());
    assert_eq!(body.as_bytes(), b"hi");
  }

  #[test]
  fn as_bytes_on_binary_returns_slice() {
    let body: RequestBody = RequestBody::Binary(vec![0xDE, 0xAD]);
    assert_eq!(body.as_bytes(), &[0xDE, 0xAD]);
  }
}

use std::collections::HashMap;

#[derive(Debug)]
pub struct Headers<'buf> {
  data: HashMap<&'buf str, &'buf str>,
}

impl<'buf> Headers<'buf> {
  pub fn get(&self, key: &str) -> Option<&&str> {
    self.data.get(key)
  }
}

impl<'buf> From<&'buf str> for Headers<'buf> {
  fn from(s: &'buf str) -> Self {
    let mut data: HashMap<&str, &str> = HashMap::new();

    for sub_str in s.split('\n') {
      if let Some((key, val)) = sub_str.split_once(':') {
        data.entry(key.trim()).or_insert(val.trim());
      }
    }

    Headers { data }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn single_header_is_parsed() {
    let h: Headers = Headers::from("Host: localhost\n");
    assert_eq!(h.get("Host"), Some(&"localhost"));
  }

  #[test]
  fn multiple_headers_are_parsed() {
    let h: Headers = Headers::from("Host: localhost\nContent-Type: text/html\n");
    assert_eq!(h.get("Host"), Some(&"localhost"));
    assert_eq!(h.get("Content-Type"), Some(&"text/html"));
  }

  #[test]
  fn missing_header_returns_none() {
    let h: Headers = Headers::from("Host: localhost\n");
    assert!(h.get("Accept").is_none());
  }

  #[test]
  fn empty_string_gives_no_headers() {
    let h: Headers = Headers::from("");
    assert!(h.get("Host").is_none());
  }

  #[test]
  fn header_with_multi_word_value_is_parsed_correctly() {
    let h: Headers = Headers::from("Content-Type: text/html; charset=utf-8\n");
    assert_eq!(h.get("Content-Type"), Some(&"text/html; charset=utf-8"));
  }

  #[test]
  fn header_value_is_trimmed() {
    let h: Headers = Headers::from("Accept:  application/json \n");
    assert_eq!(h.get("Accept"), Some(&"application/json"));
  }
}

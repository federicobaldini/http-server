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
      let mut key: &str = "";
      let mut val: &str = "";

      if let Some(i) = sub_str.find(' ') {
        key = &sub_str[..i - 1];
        val = &sub_str[i + 1..];
      }

      if key.chars().count() > 0 {
        data.entry(key).or_insert(val);
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
}

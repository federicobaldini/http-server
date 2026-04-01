use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
  data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
  Single(&'buf str),
  Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
  // Returns the raw Value enum for a key, giving access to both Single and Multiple variants
  pub fn get_value(&self, key: &str) -> Option<&Value> {
    self.data.get(key)
  }

  // Returns the value directly as &str only for single-value keys; returns None for multi-value keys or missing keys
  pub fn get(&self, key: &str) -> Option<&str> {
    match self.data.get(key) {
      Some(Value::Single(v)) => Some(v),
      _ => None,
    }
  }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
  fn from(s: &'buf str) -> Self {
    let mut data = HashMap::new();

    for sub_str in s.split('&') {
      let mut key = sub_str;
      let mut val = "";

      if let Some(i) = sub_str.find('=') {
        key = &sub_str[..i];
        val = &sub_str[i + 1..];
      }

      data
        .entry(key)
        .and_modify(|existing: &mut Value| match existing {
          Value::Single(prev_val) => {
            *existing = Value::Multiple(vec![prev_val, val]);
          }
          Value::Multiple(vec) => vec.push(val),
        })
        .or_insert(Value::Single(val));
    }
    QueryString { data }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn single_value_parameter() {
    let qs: QueryString = QueryString::from("key=value");
    match qs.get_value("key") {
      Some(Value::Single(v)) => assert_eq!(*v, "value"),
      _ => panic!("expected Single value"),
    }
  }

  #[test]
  fn two_occurrences_of_same_key_become_multiple() {
    let qs: QueryString = QueryString::from("key=a&key=b");
    match qs.get_value("key") {
      Some(Value::Multiple(v)) => {
        assert_eq!(v.len(), 2);
        assert!(v.contains(&"a"));
        assert!(v.contains(&"b"));
      }
      _ => panic!("expected Multiple values"),
    }
  }

  #[test]
  fn three_occurrences_of_same_key() {
    let qs: QueryString = QueryString::from("key=a&key=b&key=c");
    match qs.get_value("key") {
      Some(Value::Multiple(v)) => assert_eq!(v.len(), 3),
      _ => panic!("expected Multiple values"),
    }
  }

  #[test]
  fn key_without_equals_has_empty_value() {
    let qs: QueryString = QueryString::from("key");
    match qs.get_value("key") {
      Some(Value::Single(v)) => assert_eq!(*v, ""),
      _ => panic!("expected Single empty value"),
    }
  }

  #[test]
  fn multiple_distinct_parameters() {
    let qs: QueryString = QueryString::from("a=1&b=2");
    assert!(matches!(qs.get_value("a"), Some(Value::Single("1"))));
    assert!(matches!(qs.get_value("b"), Some(Value::Single("2"))));
  }

  #[test]
  fn missing_key_returns_none() {
    let qs: QueryString = QueryString::from("key=value");
    assert!(qs.get("missing").is_none());
  }

  #[test]
  fn get_returns_str_for_single_value() {
    let qs: QueryString = QueryString::from("name=alice");
    assert_eq!(qs.get("name"), Some("alice"));
  }

  #[test]
  fn get_returns_none_for_multiple_values() {
    let qs: QueryString = QueryString::from("key=a&key=b");
    assert_eq!(qs.get("key"), None);
  }

  #[test]
  fn get_returns_none_for_missing_key() {
    let qs: QueryString = QueryString::from("key=value");
    assert_eq!(qs.get("other"), None);
  }

  #[test]
  fn get_returns_empty_str_for_key_without_value() {
    let qs: QueryString = QueryString::from("key");
    assert_eq!(qs.get("key"), Some(""));
  }
}

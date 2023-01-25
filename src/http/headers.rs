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

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
    let mut data = HashMap::new();
    let mut key = "";
    let mut val = "";

    data.entry(key).or_insert(val);

    Headers { data }
  }
}

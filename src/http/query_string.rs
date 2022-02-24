use std::collections::HashMap;

pub struct QueryString<'buf> {
  data: HashMap<&'buf str, Value<'buf>>,
}

pub enum Value<'buf> {
  Single(&'buf str),
  Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
  pub fn get(&self, key: &str) -> Option<&Value> {
    self.data.get(key)
  }
}

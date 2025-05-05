#[derive(Debug, Clone)]
pub struct Name(pub String);

impl From<&str> for Name {
  fn from(s: &str) -> Self {
    Name(s.to_string())
  }
}

impl From<String> for Name {
  fn from(s: String) -> Self {
    Name(s)
  }
}

impl Default for Name {
  fn default() -> Self {
    Name("New Entity".to_string())
  }
}

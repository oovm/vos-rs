use super::*;
// impl From<Value>

#[cfg(feature = "json")]
mod json;

impl From<&str> for Object {
    fn from(value: &str) -> Self {
        Self::Text(Text { value: value.to_string() })
    }
}

impl From<String> for Object {
    fn from(value: String) -> Self {
        Self::Text(Text { value })
    }
}

impl Object {
    pub fn text() {}
}

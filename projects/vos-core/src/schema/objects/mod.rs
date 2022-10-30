use super::*;
use crate::Text;

// impl From<Value>

#[cfg(feature = "json")]
mod json;

impl From<&str> for Object {
    fn from(value: &str) -> Self {
        Object::text(value, "")
    }
}
impl From<&String> for Object {
    fn from(value: &String) -> Self {
        Object::text(value, "")
    }
}
impl From<String> for Object {
    fn from(value: String) -> Self {
        Object::text(value, "")
    }
}

impl Object {
    pub fn text(text: impl Into<String>, hint: impl Into<String>) -> Self {
        Object::Text(Text::new())
    }
}

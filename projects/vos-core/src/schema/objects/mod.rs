use serde::{Deserialize, Serialize};

use crate::{Decimal, Dict, Integer, List, Text};

mod json;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Object {
    Default,
    Boolean(bool),
    Integer(Integer),
    Decimal(Decimal),
    Reference(String),
    Text(Text),
    List(List),
    Dict(Dict),
}

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
        Object::Text(Text::new(text, hint))
    }
    pub fn integer(text: impl Into<String>, hint: impl Into<String>) -> Self {
        // Object::Integer(Text::new(text, hint))
        todo!()
    }
    pub fn decimal(text: impl Into<String>, hint: impl Into<String>) -> Self {
        // Object::Integer(Text::new(text, hint))
        todo!()
    }
}

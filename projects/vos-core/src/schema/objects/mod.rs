use super::*;

mod json;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Object {
    Default,
    Boolean(bool),
    Number(Number),
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
        Object::Text(Text { hint: hint.into(), value: text.into() })
    }
    pub fn number(number: impl Into<BigDecimal>, hint: impl Into<String>) -> Self {
        Object::Number(Number { hint: hint.into(), value: number.into() })
    }
}

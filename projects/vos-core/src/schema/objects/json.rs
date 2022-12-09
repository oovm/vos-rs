use std::str::FromStr;

use crate::{BigDecimal, Dict, Json, List, Number, Object};

impl From<&Json> for Object {
    fn from(json: &Json) -> Self {
        Object::from(json.clone())
    }
}

impl From<Json> for Object {
    fn from(json: Json) -> Self {
        match json {
            Json::Null => Object::Default,
            Json::Bool(v) => Object::Boolean(v),
            Json::Number(o) => Object::Integer(Number {
                hint: "".to_string(),
                value: BigDecimal::from_str(&o.to_string()).unwrap_or_default(),
            }),
            Json::String(o) => Object::text(o, ""),
            Json::Array(o) => Object::List(List::from_iter(o)),
            Json::Object(o) => Object::Dict(Dict::from_iter(o)),
        }
    }
}

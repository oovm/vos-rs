use crate::{Dict, Json, List, Object};

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
            Json::Number(o) => match o.as_i64() {
                Some(n) => Object::Default,
                None => match o.as_f64() {
                    Some(n) => Object::Default,
                    None => Object::Default,
                },
            },
            Json::String(o) => Object::text(o, ""),
            Json::Array(o) => Object::List(List { value: o.into_iter().map(|i| Object::from(i)).collect() }),
            Json::Object(o) => Object::Dict(Dict { value: o.into_iter().map(|(k, v)| (k.clone(), Object::from(v))).collect() }),
        }
    }
}

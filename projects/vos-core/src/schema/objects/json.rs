use crate::{schema::Object, Json};

impl From<Json> for Object {
    fn from(json: Json) -> Self {
        match json {
            Json::Null => {}
            Json::Bool(_) => {}
            Json::Number(_) => {}
            Json::String(_) => {}
            Json::Array(_) => {}
            Json::Object(_) => {}
        }
        todo!()
    }
}

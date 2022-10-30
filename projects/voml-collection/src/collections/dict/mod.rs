use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dict<T> {
    pub hint: String,
    pub dict: IndexMap<String, T>,
}

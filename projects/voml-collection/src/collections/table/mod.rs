use super::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::slice::Iter;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Table<T> {
    pub hint: String,
    pub list: Vec<T>,
    pub dict: IndexMap<String, T>,
}

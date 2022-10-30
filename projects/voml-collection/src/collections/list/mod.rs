use super::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, slice::Iter};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct List<T> {
    pub hint: String,
    pub list: VecDeque<T>,
}

impl<T> List<T> {
    pub fn push(&mut self, value: T) {
        self.list.push_back(value)
    }
    pub fn clear(&mut self) {
        self.clear()
    }
}

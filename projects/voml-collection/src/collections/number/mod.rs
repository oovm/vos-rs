use super::*;
use std::slice::Iter;

use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Number {
    pub hint: String,
    pub value: BigDecimal,
}

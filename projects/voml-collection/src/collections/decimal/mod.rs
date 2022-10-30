use super::*;
use std::slice::Iter;

mod primitive;

use bigdecimal::BigDecimal;
use num::FromPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Decimal {
    pub hint: String,
    pub value: BigDecimal,
}

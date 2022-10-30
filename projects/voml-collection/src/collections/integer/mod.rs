use num::BigInt;
use serde::{Deserialize, Serialize};

use super::*;

mod primitive;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Integer {
    pub hint: String,
    pub value: BigInt,
}

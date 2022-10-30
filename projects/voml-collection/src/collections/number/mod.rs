use std::{
    ops::{Add, Div, Mul, Rem, Sub},
    str::FromStr,
};

use bigdecimal::{num_traits::NumOps, BigDecimal, One, ParseBigDecimalError, Zero};
use num::Num;
use serde::{Deserialize, Serialize};

use super::*;

mod primitive;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Number {
    pub hint: String,
    pub value: BigDecimal,
}

impl FromStr for Number {
    type Err = ParseBigDecimalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = BigDecimal::from_str_radix(s, 10)?;
        Ok(Number { hint: "".to_string(), value })
    }
}

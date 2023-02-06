use std::str::FromStr;

use diagnostic_quick::{
    error_3rd::{num_traits::Zero, BigInt, Decimal},
    QResult,
};
use serde::{Deserialize, Serialize};

use crate::Document;

pub mod decimal_constraint;
pub mod dict_constraint;
pub mod integer_constraint;
pub mod list_constraint;
pub mod string_constraint;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SharedConstraint {
    document: Document,
}

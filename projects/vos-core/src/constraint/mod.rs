use std::str::FromStr;

use num::{BigInt, Zero};
use serde::{Deserialize, Serialize};
use vos_error::{for_3rd::BigDecimal, VosResult};
pub mod decimal_constraint;
pub mod dict_constraint;
pub mod integer_constraint;
pub mod list_constraint;
pub mod string_constraint;
use crate::Document;
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SharedConstraint {
    document: Document,
}

use std::str::FromStr;

use serde::{Deserialize, Serialize};
use vos_error::{for_3rd::BigDecimal, VosResult};
use vos_error::for_3rd::BigInt;
use vos_error::for_3rd::Zero;

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

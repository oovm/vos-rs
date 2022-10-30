use num::{BigInt, Zero};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use vos_error::VosResult;

pub mod decimal_constraint;
pub mod dict_constraint;
pub mod integer_constraint;
pub mod list_constraint;
pub mod string_constraint;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SharedConstraint {
    document_kind: String,
    document: String,
}

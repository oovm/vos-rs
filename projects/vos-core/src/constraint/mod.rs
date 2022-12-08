use std::str::FromStr;

use serde::{Deserialize, Serialize};

use diagnostic_quick::QResult;

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

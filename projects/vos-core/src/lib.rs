use serde::{Deserialize, Serialize};

use crate::schema::{Library, Service};

pub use self::constraint::{decimal_constraint::DecimalConstraint, dict_constraint::DictConstraint, integer_constraint::{IntegerConstraint, IntegerKind}, list_constraint::ListConstraint, string_constraint::StringConstraint};
pub use self::schema::Schema;

mod validator;

mod constraint;
mod schema;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Project {
    Service(Service),
    Library(Library),
}
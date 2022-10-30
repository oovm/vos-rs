use indexmap::IndexMap;
use num::BigInt;
use serde::{Deserialize, Serialize};

use crate::{IntegerConstraint, StringConstraint};

// mod parser;

#[derive(Serialize, Deserialize)]
pub struct Service {
    pub kind: ServiceKind,
}

#[derive(Serialize, Deserialize)]
pub struct Library {}

#[derive(Serialize, Deserialize)]
pub enum ServiceKind {
    Server,
    Client,
}

#[derive(Serialize, Deserialize)]
pub struct Module {
    schemas: IndexMap<String, Schema>,
    objects: IndexMap<String, Object>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Schema {
    String(StringConstraint),
    Integer(IntegerConstraint),
}

#[derive(Serialize, Deserialize)]
pub enum Object {
    Boolean(bool),
    Integer(BigInt),
    Decimal(f64),

}
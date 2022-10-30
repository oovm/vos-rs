use bigdecimal::BigDecimal;
use indexmap::IndexMap;
use num::BigInt;
use serde::{Deserialize, Serialize};

use crate::{DecimalConstraint, DictConstraint, IntegerConstraint, ListConstraint, StringConstraint};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Project {
    pub kind: ProjectKind,
    pub edition: ProjectEdition,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectEdition {
    year: i32,
    major: u8,
    patch: u8,
}

impl Default for ProjectEdition {
    fn default() -> Self {
        Self { year: 2020, major: 0, patch: 0 }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum ProjectKind {
    Server,
    Client,
    Library,
}

impl Default for ProjectKind {
    fn default() -> Self {
        Self::Library
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Module {
    schemas: IndexMap<String, Schema>,
    objects: IndexMap<String, Object>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Schema {
    String(StringConstraint),
    Integer(IntegerConstraint),
    Decimal(DecimalConstraint),
    List(ListConstraint),
    Dict(DictConstraint),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Object {
    Boolean(bool),
    Integer(BigInt),
    Decimal(BigDecimal),
    Reference(String),
    List(List),
    Dict(Dict),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct List {
    pub value: Vec<Object>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dict {
    pub value: IndexMap<String, Object>,
}

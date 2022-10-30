use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};

use bigdecimal::BigDecimal;
use indexmap::IndexMap;
use num::BigInt;
use serde::{Deserialize, Serialize};

use crate::*;
use email_address::EmailAddress;
use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
};
use url::Url;
use uuid::Uuid;
pub mod authors;
pub mod document;
pub mod edition;
pub mod license;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Project {
    pub kind: ProjectKind,
    pub license: ProjectLicense,
    pub edition: ProjectEdition,
    pub authors: BTreeSet<ProjectAuthor>,
    pub description: Document,
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

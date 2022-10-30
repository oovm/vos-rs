use crate::*;
use indexmap::IndexMap;
use num::BigInt;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};
use vos_error::{
    for_3rd::{BigDecimal, EmailAddress},
    VosResult,
};

use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
};

pub mod authors;
pub mod document;
pub mod edition;
pub mod license;
pub mod objects;

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

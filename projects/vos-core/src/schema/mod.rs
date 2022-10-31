use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
    convert::Infallible,
    fmt::{Display, Formatter},
    str::FromStr,
};
use vos_error::for_3rd::{Url, Version};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use vos_error::{for_3rd::EmailAddress, VosResult};
use crate::*;

pub mod authors;
pub mod document;
pub mod edition;
pub mod endpoint;
pub mod environment;
pub mod license;
pub mod objects;
use vos_error::VosError;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    pub kind: ProjectKind,
    pub license: ProjectLicense,
    pub edition: Version ,
    pub version: Version,
    pub authors: BTreeSet<ProjectAuthor>,
    pub description: Document,
    pub environments: Vec<Environment>,
    pub endpoints: BTreeMap<String, Endpoint>,
    pub extra: BTreeMap<String, Object>,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            kind: Default::default(),
            license: Default::default(),
            edition: Version {
                major: 2020,
                minor: 0,
                patch: 0,
                pre: Default::default(),
                build: Default::default()
            },
            version: Version {
                major: 0,
                minor: 0,
                patch: 0,
                pre: Default::default(),
                build: Default::default()
            },
            authors: Default::default(),
            description: Default::default(),
            environments: vec![],
            endpoints: Default::default(),
            extra: Default::default()
        }
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

impl Project {
    #[inline]
    pub fn extra<K, V>(&mut self, key: K, value: V) -> Option<Object>
    where
        K: Into<String>,
        V: Into<Object>,
    {
        self.extra.insert(key.into(), value.into())
    }
    #[inline]
    pub fn document(&mut self, document: &str) {
        self.description.push(document)
    }
}

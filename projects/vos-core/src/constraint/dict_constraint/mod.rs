use super::*;

#[derive(Debug, Clone)]
pub struct DictConstraint {
    /// Minimum length of utf8 string
    pub min_length: Option<u32>,
    /// Maximum length of utf8 string
    pub max_length: Option<u32>,
}


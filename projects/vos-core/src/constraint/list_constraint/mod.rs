use super::*;
use crate::List;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ListConstraint {
    /// Minimum length of utf8 string
    pub min_length: Option<u32>,
    /// Maximum length of utf8 string
    pub max_length: Option<u32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub examples: Vec<List>,
    #[serde(flatten)]
    pub info: SharedConstraint,
}

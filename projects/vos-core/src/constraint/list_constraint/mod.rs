use super::*;



#[derive(Debug, Clone)]
pub struct ListConstraint {
    /// Minimum length of utf8 string
    pub min_length: Option<u32>,
    /// Maximum length of utf8 string
    pub max_length: Option<u32>,
}
impl Default for ListConstraint {
    fn default() -> Self {
        Self { min_length: None, max_length: None }
    }
}

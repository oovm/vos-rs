use super::*;



#[derive(Debug, Clone)]
pub struct DecimalConstraint {
    /// Minimum length of utf8 string
    pub min: Option<BigInt>,
    /// Minimum number of unicode characters
    pub min_inclusive: bool,
    /// Maximum length of utf8 string
    pub max: Option<BigInt>,
    /// Maximum number of unicode characters
    pub max_inclusive: bool,
}
impl Default for DecimalConstraint {
    fn default() -> Self {
        Self { min: None, min_inclusive: true, max: None, max_inclusive: true }
    }
}

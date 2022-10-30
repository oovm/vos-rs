use super::*;
use bigdecimal::BigDecimal;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DecimalConstraint {
    pub kind: DecimalKind,
    /// Minimum length of utf8 string
    pub min: Option<BigDecimal>,
    /// Minimum number of unicode characters
    pub min_inclusive: bool,
    /// Maximum length of utf8 string
    pub max: Option<BigDecimal>,
    /// Maximum number of unicode characters
    pub max_inclusive: bool,
    #[serde(flatten)]
    pub info: SharedConstraint,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum DecimalKind {
    Decimal,
    Decimal8,
    Decimal16,
    Decimal32,
    Decimal64,
    Decimal128,
}

impl Default for DecimalKind {
    fn default() -> Self {
        Self::Decimal32
    }
}

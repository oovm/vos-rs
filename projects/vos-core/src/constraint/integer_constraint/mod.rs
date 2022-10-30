use serde::{Deserialize, Serialize};

use super::*;

/// ```vos
/// n: i32[=1]
/// n: i32[<1]
/// n: i32[1..=2]
/// n: i32[1 < n < 2]
/// ```
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct IntegerConstraint {
    pub kind: IntegerKind,
    /// Minimum length of utf8 string
    pub min: Option<BigInt>,
    /// Maximum length of utf8 string
    pub max: Option<BigInt>,
    /// Minimum number of unicode characters
    pub min_length: Option<BigInt>,
    /// Maximum number of unicode characters
    pub max_length: Option<BigInt>,
    /// Check if number is multiple of `x`
    pub multiple_of: Option<BigInt>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum IntegerKind {
    Integer,
    Integer8,
    Integer16,
    Integer32,
    Integer64,
    Integer128,
    Integer256,
    Unsigned8,
    Unsigned16,
    Unsigned32,
    Unsigned64,
    Unsigned128,
    Unsigned256,
}

impl Default for IntegerKind {
    fn default() -> Self {
        Self::Integer32
    }
}


impl IntegerConstraint {}

impl IntegerConstraint {
    /// ```vos
    /// i: i32[>1];
    /// i: i32[>=2];
    /// type Integer: i32 {
    ///     .min: -1
    /// }
    /// ```
    pub fn min(&mut self, n: &str, inclusive: bool) -> VosResult {
        let mut limit = BigInt::from_str(n)?;
        if !inclusive {
            limit += 1;
        }
        self.min = Some(limit);
        Ok(())
    }
    /// ```vos
    /// i: i32[<1];
    /// i: i32[<=0];
    /// type Integer: i32 {
    ///     .max: +1
    /// }
    /// ```
    pub fn max(mut self, n: &str, inclusive: bool) -> VosResult {
        let mut limit = BigInt::from_str(n)?;
        if !inclusive {
            limit -= 1;
        }
        self.max = Some(limit);
        Ok(())
    }
    /// ```vos
    /// type Positive: i32 {
    ///     .positive
    /// }
    /// ```
    pub fn positive(mut self) -> VosResult {
        self.min = Some(BigInt::zero());
        Ok(())
    }
    /// ```vos
    /// type Positive: i32 {
    ///     .positive
    /// }
    /// ```
    pub fn negative(mut self) -> VosResult {
        self.max = Some(BigInt::zero());
        Ok(())
    }
}

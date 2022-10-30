use std::fmt::Display;

#[cfg(feature = "bigdecimal")]
pub use bigdecimal::BigDecimal;
use diagnostic::{DiagnosticError, DiagnosticLevel};
#[cfg(feature = "email_address")]
pub use email_address::EmailAddress;
#[cfg(feature = "num")]
pub use num::BigInt;
#[cfg(feature = "url")]
pub use url::Url;

#[cfg(feature = "serde_json")]
pub use serde_json::Value as Json;

use crate::{VosError, VosErrorKind};

#[cfg(feature = "bigdecimal")]
mod for_big_decimal;
#[cfg(feature = "num")]
mod for_num;
#[cfg(feature = "peginator")]
mod for_peg;

#[cfg(feature = "email_address")]
mod for_email;

impl From<DiagnosticError> for VosError {
    fn from(error: DiagnosticError) -> Self {
        Self::runtime_error(error.to_string())
    }
}

impl VosError {
    pub fn runtime_error<S>(msg: S) -> Self
    where
        S: Display,
    {
        Self {
            kind: Box::new(VosErrorKind::RuntimeError(msg.to_string())),
            level: DiagnosticLevel::Error,
            file: Default::default(),
        }
    }
}

use diagnostic::DiagnosticLevel;
use url::ParseError;

use crate::{VosError, VosErrorKind};

impl From<ParseError> for VosError {
    fn from(error: ParseError) -> Self {
        // let p = error.position as u32;
        let e = error.to_string();
        Self { kind: Box::new(VosErrorKind::ParseError(e)), level: DiagnosticLevel::Error, file: Default::default() }
    }
}

use diagnostic::DiagnosticLevel;
use email_address::Error;

use crate::{VosError, VosErrorKind};

impl From<Error> for VosError {
    fn from(error: Error) -> Self {
        // let p = error.position as u32;
        // let e = error.specifics.to_string();
        let e = error.to_string();
        Self { kind: Box::new(VosErrorKind::ParseError(e)), level: DiagnosticLevel::Error, file: Default::default() }
    }
}

use diagnostic::DiagnosticLevel;

use semver::Error;

use crate::{VosError, VosErrorKind};

impl From<Error> for VosError {
    fn from(error: Error) -> Self {
        let e = error.to_string();
        Self { kind: Box::new(VosErrorKind::ParseError(e)), level: DiagnosticLevel::Error, file: Default::default() }
    }
}

use crate::{Codegen, Project};
use vos_error::{Validation, VosResult};

pub struct PrettyPrinter {}

impl Codegen for PrettyPrinter {
    type Output = String;

    fn generate(&self, project: &Project) -> Validation<Self::Output> {
        let mut ctx = Context::default();
        ctx.visit_root(project).ok();
        Validation::Success { value: ctx.buffer, diagnostics: vec![] }
    }
}

struct Context {
    buffer: String,
}

impl Default for Context {
    fn default() -> Self {
        Self { buffer: "".to_string() }
    }
}

impl Context {
    pub fn visit_root(&mut self, root: &Project) -> VosResult {
        Ok(())
    }
}

use core::fmt::{self, Write};
use indenter::indented;
use std::error::Error;

struct ErrorReporter<'a>(&'a dyn Error);

impl fmt::Debug for ErrorReporter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut source = Some(self.0);
        let mut i = 0;

        while let Some(error) = source {
            writeln!(f)?;
            write!(indented(f).ind(i), "{}", error)?;

            source = error.source();
            i += 1;
        }

        Ok(())
    }
}

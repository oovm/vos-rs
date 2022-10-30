use std::fmt::{Arguments, Display, Formatter, Write};

use indenter::CodeFormatter;
use vos_error::Validation;

use crate::{Codegen, Project};

pub struct PrettyPrinter {}

impl Codegen for PrettyPrinter {
    type Output = String;

    fn generate(&self, project: &Project) -> Validation<Self::Output> {
        Validation::Success { value: project.to_string(), diagnostics: vec![] }
    }
}

struct Context<'s, T>
where
    T: Write,
{
    buffer: CodeFormatter<'s, T>,
}

impl<'s, T: Write> Write for Context<'s, T> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buffer.write_str(s)
    }
    fn write_char(&mut self, c: char) -> std::fmt::Result {
        self.buffer.write_char(c)
    }
    fn write_fmt(self: &mut Self, args: Arguments<'_>) -> std::fmt::Result {
        self.buffer.write_fmt(args)
    }
}

impl<'s, T> Context<'s, T>
where
    T: Write,
{
    pub fn new(s: &'s mut T) -> Context<'s, T> {
        Self { buffer: CodeFormatter::new(s, "    ") }
    }

    pub fn visit_root(&mut self, root: &Project) -> std::fmt::Result {
        write!(self, "{}", root.description)?;
        self.write_str("service {")?;

        Ok(())
    }
}

impl Display for Project {
    /// ```vos
    /// /// dies
    /// service {
    ///     a:x
    /// }
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut ctx = Context::new(f);
        ctx.visit_root(self)
    }
}
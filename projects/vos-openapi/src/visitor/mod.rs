use openapiv3::{Contact, ExternalDocumentation, Info, License, OpenAPI, Paths};

use vos_core::{Parser, Project, ProjectAuthor, ProjectLicense, Validation, VosError};

use crate::FromOpenAPI;

mod info;
mod path;

impl Parser<OpenAPI> for FromOpenAPI {
    fn parse(&self, source: &OpenAPI) -> Validation<Project> {
        let mut ctx = Context { project: Default::default(), errors: vec![] };
        source.visit(&mut ctx);
        Validation::Success { value: ctx.project, diagnostics: vec![] }
    }
}

struct Context {
    project: Project,
    errors: Vec<VosError>,
}

trait Visit {
    type Output;
    fn visit(&self, ctx: &mut Context) -> Self::Output;
}

impl Visit for OpenAPI {
    type Output = ();

    fn visit(&self, ctx: &mut Context) -> Self::Output {
        self.info.visit(ctx);
        match &self.external_docs {
            None => {}
            Some(std) => std.visit(ctx),
        }
        self.paths.visit(ctx);
    }
}

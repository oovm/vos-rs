use std::str::FromStr;

use openapiv3::{Contact, ExternalDocumentation, Info, License, OpenAPI, Paths, Server};

use vos_core::{Environment, Parser, Project, ProjectAuthor, ProjectLicense, Url, Validation, VosError, VosResult};

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
        if let Some(s) = &self.external_docs {
            let doc = s.visit(ctx);
            ctx.project.document(&doc)
        }
        for server in &self.servers {
            match server.visit(ctx) {
                Ok(o) => ctx.project.environment(o),
                Err(e) => ctx.errors.push(e),
            }
        }
        self.paths.visit(ctx);
    }
}

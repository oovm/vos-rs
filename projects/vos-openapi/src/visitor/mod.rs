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
        match &self.external_docs {
            None => {}
            Some(std) => std.visit(ctx),
        }
        for server in &self.servers {
            match server.visit(ctx) {
                Ok(o) => ctx.project.environments,
                Err(_) => {}
            }
        }
        self.paths.visit(ctx);
    }
}

impl Visit for Server {
    type Output = VosResult<Environment>;

    fn visit(&self, _: &mut Context) -> Self::Output {
        let mut out = Environment::new(Url::from_str(&self.url)?);

        if let Some(std) = &self.description {
            out.document.push(std)
        }

        Ok(out)
    }
}

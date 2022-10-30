use openapiv3::{Contact, Info, License, OpenAPI, Paths};

use vos_core::{Document, Parser, Project, ProjectAuthor, ProjectLicense, Validation, VosError, VosResult};

use crate::FromOpenAPI;

mod info;

impl Parser<OpenAPI> for FromOpenAPI {
    fn parse(&self, source: &OpenAPI) -> Validation<Project> {
        let mut ctx = Context { project: Default::default(), errors: vec![] };
        source.visit(&mut ctx).unwrap();
        Validation::Success { value: ctx.project, diagnostics: vec![] }
    }
}

struct Context {
    project: Project,
    errors: Vec<VosError>,
}

trait Visit {
    fn visit(&self, ctx: &mut Context) -> VosResult;
}

impl Visit for OpenAPI {
    fn visit(&self, ctx: &mut Context) -> VosResult {
        self.info.visit(ctx)?;
        self.paths.visit(ctx)?;

        Ok(())
    }
}

impl Visit for Paths {
    fn visit(&self, ctx: &mut Context) -> VosResult {
        println!("{:#?}", self);
        Ok(())
    }
}

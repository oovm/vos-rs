use openapiv3::{Info, OpenAPI, Paths};
use vos_error::{Validation, VosResult};

use vos_core::{Document, Parser, Project};

use crate::FromOpenAPI;

impl Parser<OpenAPI> for FromOpenAPI {
    fn parse(&self, source: &OpenAPI) -> Validation<Project> {
        let mut ctx = Context { project: Default::default() };
        source.visit(&mut ctx).unwrap();
        Validation::Success { value: ctx.project, diagnostics: vec![] }
    }
}

struct Context {
    project: Project,
}

trait Visit {
    fn visit(&self, ctx: &mut Context) -> VosResult;
}

impl Visit for OpenAPI {
    fn visit(&self, ctx: &mut Context) -> VosResult {
        self.info.visit(ctx)?;

        Ok(())
    }
}

impl Visit for Info {
    fn visit(&self, ctx: &mut Context) -> VosResult {
        println!("{:#?}", self);

        match &self.description {
            Some(description) => {
                ctx.project.description = Document::markdown(format!("# {}\n{}", self.title, description));
            }
            None => {
                ctx.project.description = Document::markdown(format!("# {}", self.title));
            }
        }

        Ok(())
    }
}

impl Visit for Paths {
    fn visit(&self, ctx: &mut Context) -> VosResult {
        println!("{:#?}", self);
        Ok(())
    }
}

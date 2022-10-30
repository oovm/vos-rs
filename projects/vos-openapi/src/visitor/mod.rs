use openapiv3::{Info, OpenAPI};
use vos_error::VosResult;

use vos_core::Project;

use crate::FromOpenAPI;

impl FromOpenAPI {
    pub fn convert(&self, input: &OpenAPI) -> Project {
        let mut ctx = Context { project: Default::default() };
        ctx.visit_root(&input);
        ctx.project
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
        println!("{:#?}", self)
    }
}

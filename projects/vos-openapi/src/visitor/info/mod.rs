use super::*;

impl Visit for Info {
    fn visit(&self, ctx: &mut Context) -> VosResult {
        println!("{:#?}", self);

        if let Some(v) = &self.contact {
            if let Err(e) = v.visit(ctx) {
                ctx.errors.push(e)
            }
        }
        if let Some(v) = &self.license {
            if let Err(e) = v.visit(ctx) {
                ctx.errors.push(e)
            }
        }

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

impl Visit for Contact {
    fn visit(&self, ctx: &mut Context) -> VosResult {
        ProjectAuthor { name: "".to_string(), email: (), extra: Default::default() };
        Ok(())
    }
}

impl Visit for License {
    fn visit(&self, ctx: &mut Context) -> VosResult {
        ProjectLicense {};

        Ok(())
    }
}

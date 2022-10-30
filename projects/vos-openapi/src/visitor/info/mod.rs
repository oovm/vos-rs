use std::str::FromStr;

use vos_core::EmailAddress;

use super::*;

impl Visit for Info {
    fn visit(&self, ctx: &mut Context) -> VosResult {
        match &self.description {
            Some(description) => {
                ctx.project.description = Document::markdown(format!("# {}\n{}", self.title, description));
            }
            None => {
                ctx.project.description = Document::markdown(format!("# {}", self.title));
            }
        }
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
        if let Some(value) = &self.terms_of_service {
            ctx.project.extra("terms_of_service", value);
        }
        for (key, value) in &self.extensions {
            ctx.project.extra(key, value);
        }
        Ok(())
    }
}

impl Visit for Contact {
    fn visit(&self, ctx: &mut Context) -> VosResult {
        let name = match &self.email {
            Some(s) => s.clone(),
            None => return Err(VosError::parse_error("Project author missing name")),
        };
        let email = match &self.email {
            Some(s) => EmailAddress::from_str(s)?,
            None => return Err(VosError::parse_error("Project author missing email")),
        };
        let mut author = ProjectAuthor { name, email, extra: Default::default() };
        if let Some(s) = &self.url {
            author.insert("homepage", s);
        }
        for (key, value) in &self.extensions {
            author.insert(key, value);
        }
        ctx.project.authors.insert(author);
        Ok(())
    }
}

impl Visit for License {
    fn visit(&self, ctx: &mut Context) -> VosResult {
        ctx.project.license = ProjectLicense::Unknown;

        Ok(())
    }
}

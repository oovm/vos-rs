use std::{collections::BTreeMap, str::FromStr};

use vos_core::{EmailAddress, Object};

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
        if let Some(v) = &self.terms_of_service {
            // if let Err(e) = v.visit(ctx) {
            //     ctx.errors.push(e)
            // }
        }
        for (key, value) in &self.extensions {}
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
        let mut extra = BTreeMap::default();
        if let Some(s) = &self.url {
            extra.insert("homepage".clone(), Object::from(s.clone()));
        }
        for (key, value) in &self.extensions {
            extra.insert(key.clone(), Object::from(value.clone()));
        }
        ctx.project.authors.insert(ProjectAuthor { name, email, extra });
        Ok(())
    }
}

impl Visit for License {
    fn visit(&self, ctx: &mut Context) -> VosResult {
        ctx.project.license = ProjectLicense::Unknown;

        Ok(())
    }
}

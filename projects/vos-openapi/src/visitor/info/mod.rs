use super::*;

impl Visit for Info {
    type Output = ();

    fn visit(&self, ctx: &mut Context) -> Self::Output {
        ctx.project.document(&format!("# {}", self.title));
        if let Some(s) = &self.description {
            ctx.project.document(s)
        }
        if let Some(v) = &self.contact {
            v.visit(ctx)
        }
        if let Some(v) = &self.license {
            v.visit(ctx)
        }
        if let Some(value) = &self.terms_of_service {
            ctx.project.extra("terms_of_service", value);
        }
        for (key, value) in &self.extensions {
            ctx.project.extra(key, value);
        }
    }
}

impl Visit for Contact {
    type Output = ();

    fn visit(&self, ctx: &mut Context) -> Self::Output {
        let name = match &self.name {
            Some(s) => s.as_str(),
            None => "",
        };
        let email = match &self.email {
            Some(s) => s.as_str(),
            None => "",
        };
        let mut author = match ProjectAuthor::new(name, email) {
            Ok(s) => s,
            Err(e) => {
                ctx.errors.push(e);
                return;
            }
        };
        if let Some(s) = &self.url {
            author.insert("homepage", s);
        }
        for (key, value) in &self.extensions {
            author.insert(key, value);
        }
        ctx.project.authors.insert(author);
    }
}

impl Visit for ExternalDocumentation {
    type Output = ();

    fn visit(&self, ctx: &mut Context) -> Self::Output {
        if let Some(s) = &self.description {
            ctx.project.document(s)
        }
        ctx.project.document(&self.url);
        for (key, value) in &self.extensions {
            println!("Drop external document {}: {:?}", key, value)
        }
    }
}

impl Visit for License {
    type Output = ();

    fn visit(&self, ctx: &mut Context) -> Self::Output {
        ctx.project.license = ProjectLicense::from_str(&self.name);
    }
}

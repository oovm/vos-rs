use super::*;

impl Visit for Info {
    type Output = ();

    fn visit(&self, ctx: &mut Context) -> Self::Output {
        ctx.project.document(&format!("# {}", self.title));
        if let Some(s) = &self.description {
            ctx.project.document(s)
        }
        if let Some(v) = &self.contact {
            match v.visit(ctx) {
                Ok(o) => {
                    ctx.project.authors.insert(o);
                }
                Err(e) => ctx.errors.push(e),
            }
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
    type Output = QResult<ProjectAuthor>;

    fn visit(&self, _: &mut Context) -> Self::Output {
        let mut author = ProjectAuthor::new(
            self.name.as_ref().unwrap_or(&String::new()).as_str(),
            self.email.as_ref().unwrap_or(&String::new()).as_str(),
        )?;
        if let Some(s) = &self.url {
            author.insert("homepage", s);
        }
        for (key, value) in &self.extensions {
            author.insert(key, value);
        }
        Ok(author)
    }
}

impl Visit for License {
    type Output = ();

    fn visit(&self, ctx: &mut Context) -> Self::Output {
        let url = match &self.url {
            Some(s) => match Url::parse(s) {
                Ok(s) => Some(s),
                Err(_) => None,
            },
            None => None,
        };
        ctx.project.license = License::parse(&self.name, url, "");
    }
}

impl Visit for Server {
    type Output = QResult<Environment>;

    fn visit(&self, _: &mut Context) -> Self::Output {
        let mut out = Environment::new(Url::from_str(&self.url)?);

        if let Some(std) = &self.description {
            out.document.push(std)
        }

        Ok(out)
    }
}

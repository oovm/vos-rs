use std::str::FromStr;

use openapiv3::{Components, Contact, ExternalDocumentation, Info, License, OpenAPI, Paths, ReferenceOr, RequestBody, Server};

use vos_core::{Document, Environment, License, Parser, Project, ProjectAuthor, QResult, Url, Validation, VosError};

use crate::FromOpenAPI;

mod info;
mod path;

impl Parser<OpenAPI> for FromOpenAPI {
    fn parse(&self, source: &OpenAPI) -> Validation<Project> {
        let mut ctx = Context::default();
        source.visit(&mut ctx);
        Validation::Success { value: ctx.project, diagnostics: vec![] }
    }
}

#[derive(Default)]
struct Context {
    project: Project,
    errors: Vec<VosError>,
    components: Components,
}

trait Visit {
    type Output;
    fn visit(&self, ctx: &mut Context) -> Self::Output;
}

impl Visit for OpenAPI {
    type Output = ();

    fn visit(&self, ctx: &mut Context) -> Self::Output {
        if let Some(std) = &self.components {
            ctx.components = std.clone()
        }
        self.info.visit(ctx);
        read_document(&mut ctx.project.description, &None, &self.external_docs);
        for server in &self.servers {
            match server.visit(ctx) {
                Ok(o) => ctx.project.environment(o),
                Err(e) => ctx.errors.push(e),
            }
        }
        self.paths.visit(ctx);
    }
}
impl Context {
    pub fn resolve_request_bodies(&self, key: &Option<ReferenceOr<RequestBody>>) -> Option<RequestBody> {
        match key.as_ref()? {
            ReferenceOr::Item(s) => Some(s.clone()),
            ReferenceOr::Reference { reference } => {
                let (_, key) = reference.split_once("#/components/requestBodies/")?;
                match self.components.request_bodies.get(key)? {
                    ReferenceOr::Reference { reference } => external_error(reference),
                    ReferenceOr::Item(s) => Some(s.clone()),
                }
            }
        }
    }
}

fn external_error<T>(reference: &str) -> Option<T> {
    eprintln!("Can not resolve external reference {reference}");
    None
}

fn read_document(doc: &mut Document, description: &Option<String>, external_docs: &Option<ExternalDocumentation>) {
    if let Some(s) = description.as_ref() {
        doc.push(s)
    }
    if let Some(ex) = external_docs.as_ref() {
        for _ in &ex.extensions {
            // drop
        }
        let s = match &ex.description {
            None => format!("{}", ex.url),
            Some(s) => format!("{}\n{}", s, ex.url),
        };
        doc.push(&s)
    }
}

use openapiv3::{MediaType, Operation, PathItem, ReferenceOr, RequestBody, Response, Responses};

use vos_core::{Document, Endpoint};

use super::*;

impl Visit for Paths {
    type Output = ();

    fn visit(&self, ctx: &mut Context) -> Self::Output {
        for (key, value) in &self.paths {
            println!("{}", key);
            match value {
                ReferenceOr::Reference { reference } => {
                    todo!("{} need resolve", reference)
                }
                ReferenceOr::Item(v) => {
                    v.visit(ctx);
                }
            }
        }
    }
}

impl Visit for PathItem {
    type Output = Endpoint;

    fn visit(&self, ctx: &mut Context) -> Self::Output {
        let mut ep = Endpoint::default();
        match &self.description {
            None => {}
            Some(s) => ep.document.push(s),
        }
        if let Some(s) = &self.get {
            println!("{:#?}", s.visit(ctx));
        }
        if let Some(s) = &self.put {
            println!("{:#?}", s.visit(ctx));
        }
        if let Some(s) = &self.post {
            println!("{:#?}", s.visit(ctx));
        }
        if let Some(s) = &self.options {
            println!("{:#?}", s.visit(ctx));
        }

        ep
    }
}

impl Visit for Operation {
    type Output = ApiOperation;

    fn visit(&self, ctx: &mut Context) -> Self::Output {
        let mut api = ApiOperation::default();
        for _ in &self.tags {
            // drop
        }
        if let Some(s) = &self.summary {
            api.document(s)
        }
        read_document(&mut api.description, &self.description, &self.external_docs);
        api.deprecated(self.deprecated);
        if let Some(s) = ctx.resolve_request_bodies(&self.request_body) {
            s.visit(ctx)
        }
        self.responses.visit(ctx);

        // if let Some(s) = &self.responses {
        //     api.document(&s.visit(ctx))
        // }
        api
    }
}

impl Visit for RequestBody {
    type Output = ();

    fn visit(&self, ctx: &mut Context) -> Self::Output {
        let mut out = HttpRequest::default();
        read_document(&mut out.description, &self.description, &None);
        // self.required
        match &self.content {
            None => {}
            Some(_) => {}
        }
        println!("{:#?}", self);
        todo!()
    }
}

impl Visit for Responses {
    type Output = ();

    fn visit(&self, ctx: &mut Context) -> Self::Output {
        for _ in &self.extensions {
            // drop
        }
        match &self.default {
            Some(res) => match res {
                ReferenceOr::Reference { reference } => {
                    todo!("{} need resolve", reference)
                }
                ReferenceOr::Item(res) => res.visit(ctx),
            },
            None => {}
        }
        for (code, res) in &self.responses {
            match res {
                ReferenceOr::Reference { reference } => {
                    todo!("{} need resolve", reference)
                }
                ReferenceOr::Item(res) => res.visit(ctx),
            }
        }
        todo!()
    }
}

impl Visit for Response {
    type Output = ();

    fn visit(&self, ctx: &mut Context) -> Self::Output {
        println!("{:#?}", self);
        todo!()
    }
}

impl Visit for MediaType {
    type Output = ();

    fn visit(&self, ctx: &mut Context) -> Self::Output {
        todo!()
    }
}

#[derive(Default, Debug)]
pub struct ApiOperation {
    pub state: ApiState,
    pub description: Document,
}

impl ApiOperation {
    pub fn deprecated(&mut self, state: bool) {
        self.state.deprecated = state;
    }
    pub fn published(&mut self, state: bool) {
        self.state.published = state;
    }
    pub fn hidden(&mut self, state: bool) {
        self.state.hidden = state;
    }
    pub fn document(&mut self, text: &str) {
        self.description.push(text)
    }
}

#[derive(Default, Debug)]
pub struct ApiState {
    hidden: bool,
    published: bool,
    deprecated: bool,
}

#[derive(Default, Debug)]
pub struct HttpRequest {
    pub description: Document,
}

#[derive(Default, Debug)]
pub struct HttpResponse {
    pub description: Document,
}

impl HttpResponse {
    pub fn document(&mut self, text: &str) {
        self.description.push(text)
    }
}

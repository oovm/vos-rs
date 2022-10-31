use openapiv3::{Operation, PathItem, ReferenceOr, Response, Responses};

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
        if let Some(s) = &self.description {
            api.document(s)
        }
        if let Some(s) = &self.external_docs {
            api.document(&s.visit(ctx))
        }

        api.deprecated(self.deprecated);

        api
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

impl Visit for Responses {
    type Output = ();

    fn visit(&self, ctx: &mut Context) -> Self::Output {
        todo!()
    }
}

impl Visit for Response {
    type Output = ();

    fn visit(&self, ctx: &mut Context) -> Self::Output {
        todo!()
    }
}

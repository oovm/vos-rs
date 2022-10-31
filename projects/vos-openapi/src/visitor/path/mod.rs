use openapiv3::{PathItem, ReferenceOr};

use vos_core::Endpoint;

use super::*;

impl Visit for Paths {
    type Output = ();

    fn visit(&self, ctx: &mut Context) -> Self::Output {
        for (key, value) in &self.paths {
            println!("{}", key);
            println!("{:#?}", value);
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

        println!("{:#?}", self);
        ep
    }
}

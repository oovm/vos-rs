use openapiv3::{PathItem, ReferenceOr};

use super::*;

impl Visit for Paths {
    fn visit(&self, ctx: &mut Context) {
        for (key, value) in &self.paths {
            println!("{}", key);
            println!("{:#?}", value);
            match value {
                ReferenceOr::Reference { reference } => {
                    todo!("{} need resolve", reference)
                }
                ReferenceOr::Item(v) => v.visit(ctx),
            }
        }
    }
}

impl Visit for PathItem {
    fn visit(&self, ctx: &mut Context) -> Self::Output {
        todo!()
    }
}

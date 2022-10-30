pub use self::{
    constraint::{
        decimal_constraint::DecimalConstraint,
        dict_constraint::DictConstraint,
        integer_constraint::{IntegerConstraint, IntegerKind},
        list_constraint::ListConstraint,
        string_constraint::StringConstraint,
        SharedConstraint,
    },
    schema::{
        document::{Document, DocumentKind},
        Dict, List, Project, ProjectEdition, ProjectKind, Schema,
    },
};
use vos_error::{FileID, Validation};

mod constraint;
mod pretty_print;
mod schema;

pub trait Validator {}

pub trait Faker {}

pub trait Parser<S> {
    fn parse(&self, source: &S, file: &FileID) -> Validation<Project>;
}

pub trait Codegen {
    type Output;
    fn generate(&self, project: &Project) -> Validation<Self::Output>;
}

pub use voml_collection::Text;
pub use vos_error::{for_3rd::*, Validation, VosError, VosErrorKind, VosResult};

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
        authors::ProjectAuthor,
        document::{Document, DocumentKind},
        edition::ProjectEdition,
        license::ProjectLicense,
        Object, Project, ProjectKind, Schema,
    },
};

pub type List = voml_collection::List<Object>;
pub type Dict = voml_collection::Dict<Object>;

mod constraint;
mod pretty_print;
mod schema;
pub mod validator;

pub trait Faker {}

pub trait Parser<S> {
    fn parse(&self, source: &S) -> Validation<Project>;
}

pub trait Codegen {
    type Output;
    fn generate(&self, project: &Project) -> Validation<Self::Output>;
}

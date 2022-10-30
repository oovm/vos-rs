use super::*;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Document {
    pub kind: DocumentKind,
    pub text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DocumentKind {
    GFM,
}

impl Default for DocumentKind {
    fn default() -> Self {
        Self::GFM
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in self.text.lines() {
            writeln!(f, "/// {}", line)?
        }
        Ok(())
    }
}

impl Document {
    pub fn markdown(text: impl Into<String>) -> Self {
        Self { kind: DocumentKind::GFM, text: text.into() }
    }
}
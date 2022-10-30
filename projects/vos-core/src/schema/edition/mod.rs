use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectEdition {
    pub year: i32,
    pub minor: u8,
    pub patch: u8,
}

impl Default for ProjectEdition {
    fn default() -> Self {
        Self { year: 2020, minor: 0, patch: 0 }
    }
}

impl Display for ProjectEdition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}.{}.{}\"", self.year, self.minor, self.patch)
    }
}

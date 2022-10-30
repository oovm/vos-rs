use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ProjectLicense {
    Unknown,
    Apache2,
    Custom { license: String, link: String },
}

impl Default for ProjectLicense {
    fn default() -> Self {
        ProjectLicense::Unknown
    }
}

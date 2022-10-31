use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ProjectLicense {
    Builtin {
        license: String,
    },
    Custom {
license: String,
text: String,
link: Option<Url>,
}
}

impl Default for ProjectLicense {
    fn default() -> Self {
        ProjectLicense::mit()
    }
}

impl FromStr for ProjectLicense {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let out = match s.to_lowercase().as_str() {
            "mit" => ProjectLicense::mit(),
            _ => return Err(VosError::parse_error(format!("{} is an unknown license", s))),
        };
        Ok(out)
    }
}

impl ProjectLicense {
    pub fn mit() -> Self {
        todo!()
    }
    pub fn parse(s: &str, url: Option<Url>, text: impl Into<String>) {
        match  {

        }
    }

}

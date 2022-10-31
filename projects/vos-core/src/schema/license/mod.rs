use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ProjectLicense {
    MIT,
    Apache2,
    Custom { license: String, text: String, link: Option<Url> },
}

impl Default for ProjectLicense {
    fn default() -> Self {
        ProjectLicense::MIT
    }
}

impl FromStr for ProjectLicense {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut normed = String::with_capacity(s.len());
        for char in s.chars() {
            match char {
                ' ' | '-' | '_' => continue,
                c => normed.push(c.to_ascii_lowercase()),
            }
        }
        let out = match s.to_lowercase().as_str() {
            "mit" => ProjectLicense::MIT,
            "apache2" | "apache2.0" => ProjectLicense::Apache2,
            _ => ProjectLicense::Custom { license: s.to_string(), text: "".to_string(), link: None },
        };
        Ok(out)
    }
}

impl ProjectLicense {
    pub fn parse(license: &str, url: Option<Url>, content: impl Into<String>) -> ProjectLicense {
        let mut out = Self::from_str(license).unwrap();
        if let ProjectLicense::Custom { text, link, .. } = &mut out {
            *link = url;
            *text = content.into();
        }
        out
    }
}

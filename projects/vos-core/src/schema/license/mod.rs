use super::*;

/// https://spdx.github.io/spdx-spec/v2.3/SPDX-license-list/
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum License {
    MIT,
    Apache2,
    Custom { license: String, text: String, link: Option<Url> },
}

impl Default for License {
    fn default() -> Self {
        License::MIT
    }
}

impl FromStr for License {
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
            "mit" => License::MIT,
            "apache2" | "apache2.0" => License::Apache2,
            _ => License::Custom { license: s.to_string(), text: "".to_string(), link: None },
        };
        Ok(out)
    }
}

impl License {
    pub fn parse(license: &str, url: Option<Url>, content: impl Into<String>) -> License {
        let mut out = Self::from_str(license).unwrap();
        if let License::Custom { text, link, .. } = &mut out {
            *link = url;
            *text = content.into();
        }
        out
    }
}

use super::*;

mod der;

/// https://spdx.github.io/spdx-spec/v2.3/SPDX-license-list/
#[derive(Clone, Debug, Serialize)]
pub struct License {
    license: String,
    text: String,
    link: Option<Url>,
}

impl Default for License {
    fn default() -> Self {
        License::MIT
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

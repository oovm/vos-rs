use serde::Deserializer;

use super::*;

impl<'de> Deserialize<'de> for License {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }

    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
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

struct LicenseVisitor {}

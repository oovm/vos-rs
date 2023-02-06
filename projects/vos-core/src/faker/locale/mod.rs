use super::*;

impl Default for Faker {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl Default for FakerLocale {
    fn default() -> Self {
        Self::en()
    }
}

impl FakerLocale {
    pub fn en() -> Self {
        Self {}
    }
}

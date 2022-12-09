use rand::distributions::Distribution;

mod time;

pub struct Faker {
    locale: FakerLocale,
}

impl Faker {
    pub fn new(locale: FakerLocale) -> Self {
        Self { locale }
    }
}

pub struct FakerLocale {}

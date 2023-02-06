use rand::{distributions::Distribution, rngs::SmallRng, thread_rng, SeedableRng};

mod locale;
mod time;

pub struct Faker {
    rng: SmallRng,
    locale: FakerLocale,
}

impl Faker {
    pub fn new(locale: FakerLocale) -> Self {
        let rng = SmallRng::from_rng(thread_rng()).unwrap();
        Self { rng, locale }
    }
}

pub struct FakerLocale {}

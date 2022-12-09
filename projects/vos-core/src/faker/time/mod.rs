use chrono::DateTime;
use rand::{
    distributions::{DistIter, DistMap},
    Rng,
};
use std::time::SystemTime;

use super::*;

impl Distribution<SystemTime> for Faker {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SystemTime {
        todo!()
    }

    fn sample_iter<R>(self, rng: R) -> DistIter<Self, R, SystemTime>
    where
        R: Rng,
        Self: Sized,
    {
        todo!()
    }

    fn map<F, S>(self, func: F) -> DistMap<Self, F, SystemTime, S>
    where
        F: Fn(SystemTime) -> S,
        Self: Sized,
    {
        todo!()
    }
}

// impl<T> Distribution<DateTime<T>> for Faker {
//     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DateTime<T> {
//         DateTime::from()
//     }
// }

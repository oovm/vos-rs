use chrono::{NaiveTime, Utc};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rand::{distributions::uniform::SampleRange, Rng};

use super::*;

// Duration

impl Faker {
    pub fn duration<R: SampleRange<u64>>(&mut self, range: R) -> Duration {
        let duration = self.rng.gen_range(range);
        Duration::from_millis(duration)
    }
    pub fn system_time<R: SampleRange<i64>>(&mut self, range: R) -> SystemTime {
        let duration = self.rng.gen_range(range);
        if duration < 0 {
            SystemTime::now() - Duration::from_millis(duration.abs() as u64)
        }
        else {
            SystemTime::now() + Duration::from_millis(duration as u64)
        }
    }
    pub fn unix_time<R: SampleRange<i64>>(&mut self, range: R) -> SystemTime {
        let duration = self.rng.gen_range(range);
        if duration < 0 {
            UNIX_EPOCH - Duration::from_millis(duration.abs() as u64)
        }
        else {
            UNIX_EPOCH + Duration::from_millis(duration as u64)
        }
    }
    pub fn native_time<R: SampleRange<i64>>(&mut self, range: R) -> NaiveTime {
        let duration = self.rng.gen_range(range);
        if duration < 0 {
            Utc::now().naive_utc() - Duration::from_millis(duration.abs() as u64)
        }
        else {
            Utc::now().naive_utc() + Duration::from_millis(duration as u64)
        }
    }
}

#[test]
fn test() {
    let mut faker = Faker::default();
    println!("{:?}", faker.duration(0..=100));
    println!("{:?}", faker.system_time(-100..=100));
    println!("{:?}", faker.unix_time(-100..=100));
}

// impl<T> Distribution<DateTime<T>> for Faker {
//     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DateTime<T> {
//         DateTime::from()
//     }
// }

use crate::{hal9000, multivac};

use super::Error;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

/// Implement a standard distribution for our central delay code enum.
/// With this trait, `rand::random()` can be used to get a random enum variant.
/// All members of enum variants must be default-constructible for that to work.
impl Distribution<Error> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Error {
        // This ain't great for many reasons.
        let total_count = hal9000::Error::VARIANT_COUNT + multivac::Error::VARIANT_COUNT;
        let index = rng.gen_range(0..total_count);

        if index < multivac::Error::VARIANT_COUNT {
            Error::Multivac(rand::random())
        } else {
            Error::Hal9000(rand::random())
        }
    }
}

use super::Error;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

/// Implement a standard distribution for our central delay code enum.
/// With this trait, `rand::random()` can be used to get a random enum variant.
/// All members of enum variants must be default-constructible for that to work.
impl Distribution<Error> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Error {
        let index = rng.gen_range(0..Error::VARIANT_COUNT);

        // This ain't great for many reasons.
        match index {
            0 => Error::Hal9000(rand::random()),
            1 => Error::UniversalAc(rand::random()),
            _ => unreachable!(),
        }
    }
}

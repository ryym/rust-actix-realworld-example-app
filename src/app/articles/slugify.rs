use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::iter;

register_service!(CanSlugify);

pub trait CanSlugify {
    // TODO: Implement better conversion.
    fn slugify(&self, value: &str) -> String {
        let mut rng = thread_rng();
        let random: String = iter::repeat(())
            .map(|_| rng.sample(Alphanumeric))
            .take(10)
            .collect();
        value.replace(" ", "-").to_lowercase() + "-" + &random
    }
}

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

fn main() {
    let mut rng = ChaCha8Rng::seed_from_u64(0);
    println!("Hello, world! {}", rng.random::<f64>());

}

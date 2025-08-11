use std::cell::RefCell;
use std::rc::Rc;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

fn main() {
    let mut rng = ChaCha8Rng::seed_from_u64(0);
    let unmut = RefCell::new(ChaCha8Rng::seed_from_u64(0));
    dbg!(unmut.borrow_mut().random::<u8>());
}

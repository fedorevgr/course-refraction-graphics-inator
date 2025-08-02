use std::num::Saturating;

fn main() {
    let a: Saturating<u8> = Saturating(80);
    let b: Saturating<u8> = Saturating(200);
    println!("{}", a + b);
}

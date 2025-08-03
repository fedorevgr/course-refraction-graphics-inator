use std::num::Saturating;

#[derive(Debug, Copy, Clone)]
struct ColorVal<T>(pub T);

impl std::ops::Add for ColorVal<u8> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.saturating_add(rhs.0))
    }
}

impl std::ops::Mul for ColorVal<u8> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0.wrapping_mul(rhs.0))
    }
}

impl From<u8> for ColorVal<u8> {
    fn from(v: u8) -> ColorVal<u8> {
        ColorVal(v)
    }
}

fn mul_hi(x: u8, y: u8) -> u8 {
    (((x as u16) * (y as u16)) >> 8) as u8
}

fn main() {
    let a = ColorVal(80u8);
    let b = ColorVal(200u8);
    println!("{}", (a + b).0);
    println!("{}", (a * b).0);

    let c = 80u8;
    let d = 100u8;

    println!("{}", mul_hi(c, d));
}

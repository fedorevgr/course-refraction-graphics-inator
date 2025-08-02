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
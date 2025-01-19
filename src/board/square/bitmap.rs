use crate::rules::Coordinate;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{BitAnd, BitOr, BitXor, Deref, Not};

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Bitmap(u64);

impl Bitmap {
    #[must_use]
    pub fn contains(&self, bit_index: u8) -> bool {
        self.0 & (1 << bit_index) != 0
    }
}

impl From<u64> for Bitmap {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Coordinate> for Bitmap {
    fn from(value: Coordinate) -> Self {
        Self::from(1u64 << value.bit_index())
    }
}

impl Deref for Bitmap {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for Bitmap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bitmap({:#066b})", self.0)
    }
}

impl Display for Bitmap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#066b}", self.0)
    }
}

impl BitAnd for Bitmap {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitOr for Bitmap {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitXor for Bitmap {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl Not for Bitmap {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

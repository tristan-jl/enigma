use std::convert::TryInto;
use std::ops::{Add, AddAssign, Sub};

pub const MAX_VALUE: u8 = 26;
pub type WireSize = u8;
pub type WiringSize = [WireSize; MAX_VALUE as usize];

pub fn encoding_to_wiring(encoding: &str) -> WiringSize {
    encoding
        .to_lowercase()
        .as_bytes()
        .iter()
        .map(|&b| b - 97)
        .collect::<Vec<WireSize>>()
        .try_into()
        .unwrap()
}

#[allow(dead_code)]
pub fn wiring_to_encoding(wiring: WiringSize) -> String {
    wiring.iter().map(|x| (x + 97) as char).collect()
}

#[derive(Clone, Copy, Debug)]
pub struct ClockInt {
    value: WireSize,
}

impl ClockInt {
    #[inline]
    pub fn value(&self) -> WireSize {
        self.value
    }
}

impl From<u8> for ClockInt {
    fn from(value: u8) -> Self {
        ClockInt {
            value: value.rem_euclid(MAX_VALUE),
        }
    }
}
impl Add<Self> for ClockInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.value() + rhs.value())
    }
}
impl Add<u8> for ClockInt {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        Self::from(self.value() + rhs)
    }
}
impl AddAssign<u8> for ClockInt {
    fn add_assign(&mut self, rhs: u8) {
        self.value = (self.value + rhs).rem_euclid(MAX_VALUE)
    }
}
impl Sub<Self> for ClockInt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from(
            (self.value() + MAX_VALUE - rhs.value().rem_euclid(MAX_VALUE)).rem_euclid(MAX_VALUE),
        )
    }
}
impl Sub<u8> for ClockInt {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self::Output {
        Self::from((self.value() + MAX_VALUE - rhs.rem_euclid(MAX_VALUE)).rem_euclid(MAX_VALUE))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encoding_to_wiring_works() {
        assert_eq!(
            encoding_to_wiring("abcdefghijklmnopqrstuvwxyz"),
            [
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25
            ]
        )
    }
    #[test]
    fn wiring_to_encoding_works() {
        assert_eq!(
            wiring_to_encoding([
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25
            ]),
            "abcdefghijklmnopqrstuvwxyz"
        )
    }

    #[test]
    fn clock_int_base() {
        assert_eq!(ClockInt::from(23).value(), 23)
    }
    #[test]
    fn clock_int_overflow() {
        assert_eq!(ClockInt::from(27).value(), 1)
    }

    #[test]
    fn clock_int_add() {
        let mi = ClockInt::from(23) + 7;
        assert_eq!(mi.value(), 4)
    }
    #[test]
    fn clock_int_add_assign() {
        let mut mi = ClockInt::from(23);
        mi += 34;
        assert_eq!(mi.value(), 5)
    }
    #[test]
    fn clock_int_sub() {
        let mi = ClockInt::from(6) - 18;
        assert_eq!(mi.value(), 14)
    }
}

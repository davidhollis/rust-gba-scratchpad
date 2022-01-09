use core::ops::{Add, Sub, Mul};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct UFixed8(u16);

impl UFixed8 {
    pub const FRACTION_BITS: u16 = 8;

    pub const ZERO: UFixed8 = UFixed8(0u16);

    pub fn abs_diff(self, other: UFixed8) -> UFixed8 {
        if self < other {
            other - self
        } else {
            self - other
        }
    }

    // pub fn checked_add(self, rhs: SFixed8) -> Option<UFixed8> {
    //     self.0.checked_add_signed(rhs.0).map(|v| UFixed8(v))
    // }

    #[inline]
    pub fn overflowing_add_signed(self, rhs: SFixed8) -> (UFixed8, bool) {
        let (v, overflow) = self.0.overflowing_add(rhs.0 as u16);
        (UFixed8(v), overflow ^ (rhs.0 < 0))
    }

    #[inline]
    pub fn saturating_add_signed(self, rhs: SFixed8) -> UFixed8 {
        let (res, overflow) = self.0.overflowing_add(rhs.0 as u16);
        UFixed8(
            if overflow == (rhs.0 < 0) {
                res
            } else if overflow {
                u16::MAX
            } else {
                0u16
            }
        )
    }
    

    #[inline]
    pub fn wrapping_add_signed(self, rhs: SFixed8) -> UFixed8 {
        UFixed8(self.0.wrapping_add(rhs.0 as u16))
    }
}

impl Add for UFixed8 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        UFixed8(self.0 + rhs.0)
    }
}

impl Sub for UFixed8 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        UFixed8(self.0 - rhs.0)
    }
}

impl Mul for UFixed8 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        UFixed8((self.0 * rhs.0) >> UFixed8::FRACTION_BITS)
    }
}

impl Mul<u16> for UFixed8 {
    type Output = Self;
    fn mul(self, rhs: u16) -> Self {
        UFixed8(self.0 * rhs)
    }
}

impl From<u16> for UFixed8 {
    fn from(v: u16) -> UFixed8 {
        UFixed8(v << UFixed8::FRACTION_BITS)
    }
}

impl From<UFixed8> for u16 {
    fn from(v: UFixed8) -> u16 {
        v.0 >> UFixed8::FRACTION_BITS
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct SFixed8(i16);

impl SFixed8 {
    pub const FRACTION_BITS: u16 = 8;
    pub const SCALE: u16 = 1 << SFixed8::FRACTION_BITS;

    pub const ZERO: SFixed8 = SFixed8(0i16);
}

impl Add for SFixed8 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        SFixed8(self.0 + rhs.0)
    }
}

impl Sub for SFixed8 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        SFixed8(self.0 - rhs.0)
    }
}

impl Mul for SFixed8 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        SFixed8((self.0 * rhs.0) >> SFixed8::FRACTION_BITS)
    }
}

impl Mul<i16> for SFixed8 {
    type Output = Self;
    fn mul(self, rhs: i16) -> Self {
        SFixed8(self.0 * rhs)
    }
}

impl From<i16> for SFixed8 {
    fn from(v: i16) -> SFixed8 {
        SFixed8(v << SFixed8::FRACTION_BITS)
    }
}

impl From<SFixed8> for i16 {
    fn from(v: SFixed8) -> i16 {
        v.0 >> SFixed8::FRACTION_BITS
    }
}
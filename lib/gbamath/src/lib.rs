#![no_std]

use core::ops::{ Add, AddAssign, Mul };

pub mod fixed;
pub mod geometry;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Vec2D<T> {
    pub x: T,
    pub y: T,
}

impl Vec2D<fixed::UFixed8> {
    #[inline]
    pub fn saturating_add_signed(self, rhs: Vec2D<fixed::SFixed8>) -> Self {
        Vec2D {
            x: self.x.saturating_add_signed(rhs.x),
            y: self.y.saturating_add_signed(rhs.y),
        }
    }

    #[inline]
    pub fn saturating_add_signed_assign(&mut self, rhs: Vec2D<fixed::SFixed8>) {
        self.x = self.x.saturating_add_signed(rhs.x);
        self.y = self.y.saturating_add_signed(rhs.y);
    }
}

impl<T> Add for Vec2D<T> where T: Add<T> + Add<Output = T> + Copy {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vec2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> AddAssign for Vec2D<T> where T: Add<T> + Add<Output = T> + Copy {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<T> Mul<T> for Vec2D<T> where T: Mul<T> + Mul<Output = T> + Copy {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        Vec2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
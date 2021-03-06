use crate::fixed::UFixed8;
use crate::Vec2D;

pub struct BoundingBox {
    pub center: Vec2D<UFixed8>,
    pub half_size: Vec2D<UFixed8>,
}

impl BoundingBox {
    pub fn intersects(&self, other: &BoundingBox) -> bool {
        ! (
            (self.center.x.abs_diff(other.center.x) > self.half_size.x + other.half_size.x) ||
            (self.center.y.abs_diff(other.center.y) > self.half_size.y + other.half_size.y)
        )
    }

    #[inline]
    pub fn left(&self) -> UFixed8 {
        self.center.x - self.half_size.x
    }

    #[inline]
    pub fn right(&self) -> UFixed8 {
        self.center.x + self.half_size.x
    }

    #[inline]
    pub fn top(&self) -> UFixed8 {
        self.center.y - self.half_size.y
    }

    #[inline]
    pub fn bottom(&self) -> UFixed8 {
        self.center.y + self.half_size.y
    }
}
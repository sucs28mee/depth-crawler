use std::ops::*;

use macroquad::prelude::Vec2;

#[derive(Clone, Copy, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

impl Vector2 {
    pub const ZERO: Vector2 = Vector2 { x: 0., y: 0. };
    pub const ONE: Vector2 = Vector2 { x: 1., y: 1. };
    pub const UNIT_X: Vector2 = Vector2 { x: 1., y: 0. };
    pub const UNIT_Y: Vector2 = Vector2 { x: 0., y: 1. };

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn normalized(&self) -> Vector2 {
        let mult = 1. / (self.x * self.x + self.y * self.y).sqrt();
        Vector2 { x: self.x * mult, y: self.y * mult }
    }

    pub fn dir_to(&self, other: Vector2) -> Vector2 {
        (other - *self).normalized()
    }

    pub fn to_rotation(&self) -> f32 {
        self.y.atan2(self.x)
    }

    pub fn lerp(&mut self, other: Vector2, t: f32) {
        self.x = self.x + t * (other.x - self.x);
        self.y = self.y + t * (other.y - self.y);
    }

    pub fn as_vec2(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y
        }
    }
}

impl Add<Vector2> for Vector2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl AddAssign<Vector2> for Vector2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Vector2> for Vector2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl SubAssign<Vector2> for Vector2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<Vector2> for Vector2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl MulAssign<Vector2> for Vector2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
    }
}

impl Div<Vector2> for Vector2 {
    type Output = Self;

    fn div(self, rhs: Vector2) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y
        }
    }
}

impl DivAssign<Vector2> for Vector2 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl MulAssign<f32> for Vector2 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<f32> for Vector2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}

impl DivAssign<f32> for Vector2 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Neg for Vector2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y
        }
    }
}


impl From<[f32; 2]> for Vector2 {
    #[inline]
    fn from(a: [f32; 2]) -> Self {
        Self::new(a[0], a[1])
    }
}

impl From<Vector2> for [f32; 2] {
    #[inline]
    fn from(v: Vector2) -> Self {
        [v.x, v.y]
    }
}

impl From<(f32, f32)> for Vector2 {
    #[inline]
    fn from(t: (f32, f32)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl From<Vector2> for (f32, f32) {
    #[inline]
    fn from(v: Vector2) -> Self {
        (v.x, v.y)
    }
}
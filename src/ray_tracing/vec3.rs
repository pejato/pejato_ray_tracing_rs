use std::fmt::Display;

use auto_ops::{impl_op, impl_op_ex};
use derive_more::{Add, AddAssign, Sub, SubAssign};
mod tests;

// MARK: - Data

#[derive(Clone, Copy, Debug, Add, AddAssign, Sub, SubAssign)]
pub struct Vec3 {
    pub a: f32,
    pub b: f32,
    pub c: f32,
}

// MARK: - Methods

impl Vec3 {
    pub fn new(a: f32, b: f32, c: f32) -> Self {
        Self { a, b, c }
    }

    pub fn unit(self) -> Self {
        self / self.magnitude()
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn magnitude(self) -> f32 {
        f32::sqrt(self.dot(self))
    }

    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Self::new(
            self.b * rhs.c - self.c * rhs.b,
            self.c * rhs.a - self.a * rhs.c,
            self.a * rhs.b - self.b * rhs.a,
        )
    }

    pub fn dot(self, rhs: Vec3) -> f32 {
        self.mul_elts(rhs).summed()
    }

    pub fn mul_elts(self, rhs: Vec3) -> Self {
        Self::new(self.a * rhs.a, self.b * rhs.b, self.c * rhs.c)
    }

    pub fn summed(self) -> f32 {
        self.a + self.b + self.c
    }
}

// MARK: - Vec3 + Display

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:.3}, {:.3}, {:.3}]", self.a, self.b, self.c)
    }
}

// MARK: - Operators

impl_op_ex!(*|lhs: Vec3, rhs: f32| -> Vec3 { Vec3::new(lhs.a * rhs, lhs.b * rhs, lhs.c * rhs) });
impl_op!(*= |lhs: &mut Vec3, rhs: f32| { *lhs = *lhs * rhs; });
impl_op_ex!(/ |lhs: Vec3, rhs: f32| -> Vec3 { Vec3::new(lhs.a / rhs, lhs.b / rhs, lhs.c / rhs) });
impl_op!(/= |lhs: &mut Vec3, rhs: f32| { *lhs = *lhs / rhs; });

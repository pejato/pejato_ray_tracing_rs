use std::fmt::Display;

use auto_ops::{impl_op, impl_op_ex};
mod tests;

// MARK: - Data

#[derive(Clone, Copy, Debug)]
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

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn magnitude(self) -> f32 {
        f32::sqrt(self.dot(self))
    }

    pub fn summed(self) -> f32 {
        self.a + self.b + self.c
    }

    pub fn dot(self, rhs: Vec3) -> f32 {
        self.mul_elts(rhs).summed()
    }

    pub fn mul_elts(self, rhs: Vec3) -> Self {
        Self {
            a: self.a * rhs.a,
            b: self.b * rhs.b,
            c: self.c * rhs.c,
        }
    }
}

// MARK: - Vec3 + Display

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:.3}, {:.3}, {:.3}]", self.a, self.b, self.c)
    }
}

// MARK: - Operators

impl_op_ex!(+ |lhs: Vec3, rhs: Vec3| -> Vec3 { Vec3::new(lhs.a + rhs.a, lhs.b + rhs.b, lhs.c + rhs.c) });

impl_op!(+= |lhs: &mut Vec3, rhs: Vec3| { *lhs = *lhs + rhs; });

impl_op_ex!(-|lhs: Vec3, rhs: Vec3| -> Vec3 {
    Vec3::new(lhs.a - rhs.a, lhs.b - rhs.b, lhs.c - rhs.c)
});

impl_op!(-= |lhs: &mut Vec3, rhs: Vec3| { *lhs = *lhs - rhs; });

impl_op_ex!(*|lhs: Vec3, rhs: f32| -> Vec3 { Vec3::new(lhs.a * rhs, lhs.b * rhs, lhs.c * rhs) });

impl_op!(*= |lhs: &mut Vec3, rhs: f32| { *lhs = *lhs * rhs; });

impl_op_ex!(/ |lhs: Vec3, rhs: f32| -> Vec3 { Vec3::new(lhs.a / rhs, lhs.b / rhs, lhs.c / rhs) });

impl_op!(/= |lhs: &mut Vec3, rhs: f32| { *lhs = *lhs / rhs; });

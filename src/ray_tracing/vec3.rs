use std::fmt::Display;

use auto_ops::{impl_op, impl_op_ex, impl_op_ex_commutative};
use derive_more::{Add, AddAssign, Sub, SubAssign};
use rand::Rng;
mod tests;

// MARK: - Data

#[derive(Clone, Copy, Debug, Add, AddAssign, Sub, SubAssign)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// MARK: - Methods

impl Vec3 {
    pub fn new(a: f32, b: f32, c: f32) -> Self {
        Self { x: a, y: b, z: c }
    }

    pub fn rand_in_unit_sphere() -> Self {
        // Again, this should probably use DI.
        let mut rng = rand::thread_rng();
        loop {
            let vec = Vec3::new(
                rng.gen_range(-1.0..=1.0),
                rng.gen_range(-1.0..=1.0),
                rng.gen_range(-1.0..=1.0),
            );
            if vec.dot(vec) >= 1.0 {
                continue;
            }
            return vec;
        }
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
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn dot(self, rhs: Vec3) -> f32 {
        self.mul_elts(rhs).summed()
    }

    pub fn mul_elts(self, rhs: Vec3) -> Self {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }

    pub fn summed(self) -> f32 {
        self.x + self.y + self.z
    }
}

// MARK: - Vec3 + Display

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:.3}, {:.3}, {:.3}]", self.x, self.y, self.z)
    }
}

// MARK: - Operators

impl_op_ex_commutative!(*|lhs: Vec3, rhs: f32| -> Vec3 {
    Vec3::new(lhs.x * rhs, lhs.y * rhs, lhs.z * rhs)
});
impl_op_ex!(/ |lhs: Vec3, rhs: f32| -> Vec3 { Vec3::new(lhs.x / rhs, lhs.y / rhs, lhs.z / rhs) });
impl_op_ex!(*= |lhs: &mut Vec3, rhs: f32| { *lhs = *lhs * rhs; });
impl_op_ex!(/= |lhs: &mut Vec3, rhs: f32| { *lhs = *lhs / rhs; });
impl_op!(-|vec: Vec3| -> Vec3 { Vec3::new(-vec.x, -vec.y, -vec.z) });

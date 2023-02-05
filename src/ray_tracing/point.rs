use super::vec3::Vec3;
use auto_ops::{impl_op, impl_op_ex};
use derive_more::{Add, AddAssign, From, Into, Sub, SubAssign};

mod tests;

// MARK: - Data

#[derive(Copy, Clone, Add, AddAssign, Sub, SubAssign, Into, From)]
pub struct Point(Vec3);

// MARK: - Methods

impl Point {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self(Vec3::new(r, g, b))
    }

    pub fn zero() -> Self {
        Self(Vec3::zero())
    }
}

// MARK: - Operators

impl_op_ex!(*|lhs: Point, rhs: f32| -> Point {
    Point::new(lhs.0.a * rhs, lhs.0.b * rhs, lhs.0.c * rhs)
});
impl_op!(*= |lhs: &mut Point, rhs: f32| { *lhs = *lhs * rhs; });
impl_op_ex!(/ |lhs: Point, rhs: f32| -> Point { Point::new(lhs.0.a / rhs, lhs.0.b / rhs, lhs.0.c / rhs) });
impl_op!(/= |lhs: &mut Point, rhs: f32| { *lhs = *lhs / rhs; });

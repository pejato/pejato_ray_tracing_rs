use super::vec3::Vec3;
use auto_ops::{impl_op, impl_op_ex, impl_op_ex_commutative};
use derive_more::{Add, AddAssign, From, Into, Sub, SubAssign};

mod tests;

// MARK: - Data
#[derive(Copy, Clone, Add, AddAssign, Sub, SubAssign, From, Into)]
pub struct Color(Vec3);

// MARK: - Methods

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self(Vec3::new(r, g, b))
    }
}

// MARK: - Color + Display

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let &Color(vec) = self;
        let pixel = |flt: f32| (flt * 255.999).round() as i32;
        write!(f, "{} {} {}", pixel(vec.x), pixel(vec.y), pixel(vec.z))
    }
}

// MARK: - Operators

impl_op_ex_commutative!(*|lhs: Color, rhs: f32| -> Color {
    Color::new(lhs.0.x * rhs, lhs.0.y * rhs, lhs.0.z * rhs)
});
impl_op_ex!(*= |lhs: &mut Color, rhs: f32| { *lhs = *lhs * rhs; });
impl_op_ex!(/ |lhs: Color, rhs: f32| -> Color { Color::new(lhs.0.x / rhs, lhs.0.y / rhs, lhs.0.z / rhs) });
impl_op_ex!(/= |lhs: &mut Color, rhs: f32| { *lhs = *lhs / rhs; });

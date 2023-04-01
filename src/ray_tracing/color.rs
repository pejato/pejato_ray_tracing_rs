use super::vec3::Vec3;
use auto_ops::{impl_op_ex, impl_op_ex_commutative};
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

    pub fn gamma_corrected(self) -> Self {
        // Raise color to power 1/gamma, where gamma == 2
        Color::new(self.0.x.sqrt(), self.0.y.sqrt(), self.0.z.sqrt())
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

// MARK: - Averaging

pub trait Averageable {
    type Output;
    fn averaged(self) -> Self::Output;
}

impl<Container> Averageable for Container
where
    Container: Iterator<Item = Color>,
    Container: Sized,
{
    type Output = Color;

    fn averaged(self) -> Self::Output {
        let mut total = 0.0;
        let summed = self
            .into_iter()
            .fold(Color::new(0.0, 0.0, 0.0), |acc, color| {
                // self.into_iter().count() consumes self, which means I don't know how to get count before reducing.
                // This is super gross but I don't feel like dealing with ownership problems
                total += 1.0;
                acc + color
            });
        // No div by 0's pls
        let total = if total < f32::EPSILON { 1.0 } else { total };
        summed / total
    }
}

// MARK: - Operators

impl_op_ex_commutative!(*|lhs: Color, rhs: f32| -> Color {
    Color::new(lhs.0.x * rhs, lhs.0.y * rhs, lhs.0.z * rhs)
});
impl_op_ex!(*= |lhs: &mut Color, rhs: f32| { *lhs = *lhs * rhs; });
impl_op_ex!(/ |lhs: Color, rhs: f32| -> Color { Color::new(lhs.0.x / rhs, lhs.0.y / rhs, lhs.0.z / rhs) });
impl_op_ex!(/= |lhs: &mut Color, rhs: f32| { *lhs = *lhs / rhs; });

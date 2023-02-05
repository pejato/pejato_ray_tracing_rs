use super::vec3::Vec3;

mod tests;

// MARK: - Data
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
        write!(f, "{} {} {}", pixel(vec.a), pixel(vec.b), pixel(vec.c))
    }
}

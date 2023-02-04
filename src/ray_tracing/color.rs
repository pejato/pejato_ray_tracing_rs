use super::vec3::Vec3;

pub struct Color(Vec3);

impl Color {
    pub fn new(r: i32, g: i32, b: i32) -> Self {
        Self(Vec3::new(r, g, b))
    }
}
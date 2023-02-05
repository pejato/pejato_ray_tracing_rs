use super::vec3::Vec3;

pub struct Point(Vec3);

impl Point {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self(Vec3::new(r, g, b))
    }
}

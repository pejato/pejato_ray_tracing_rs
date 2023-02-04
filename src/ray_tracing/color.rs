use super::vec3::Vec3;

pub struct Color(Vec3);

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self(Vec3::new(r, g, b))
    }
}

// TODO: Define a (proc?) macro to automatically derive impl's defined on Vec3

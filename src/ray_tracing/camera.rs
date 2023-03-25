use super::{point::Point, ray::Ray, vec3::Vec3};

pub struct Camera {
    viewport_height: f32,
    viewport_width: f32,
    focal_len: f32,
    origin: Point,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left: Point,
}

// MARK: - Camera + Methods

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio: f32 = 16.0 / 9.0;

        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_len = 1.0;
        let origin = Point::zero();
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left = origin
            - (horizontal / 2.0).into()
            - (vertical / 2.0).into()
            - Vec3::new(0.0, 0.0, focal_len).into();

        Self {
            viewport_height,
            viewport_width,
            focal_len,
            origin,
            horizontal,
            vertical,
            lower_left,
        }
    }

    pub fn ray_at(&self, horizontal_pct: f32, vertical_pct: f32) -> Ray {
        let horizontal_pct = horizontal_pct.clamp(0.0, 1.0);
        let vertical_pct = vertical_pct.clamp(0.0, 1.0);
        Ray::new(
            self.origin,
            Vec3::from(self.lower_left)
                + horizontal_pct * self.horizontal
                + vertical_pct * self.vertical
                - Vec3::from(self.origin),
        )
    }
}

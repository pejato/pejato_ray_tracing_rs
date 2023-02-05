
#[cfg(test)]
mod method_tests {
    use crate::{ray_tracing::{ray::Ray, point::Point, vec3::Vec3}, assert_point_approx_eq, assert_vec_approx_eq};

    #[test]
    fn test_new() {
        let actual = Ray::new(Point::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
        let expected = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            dir: Vec3::new(1.0, 1.0, 1.0)
        };
        assert_point_approx_eq!(actual.origin, expected.origin);
        assert_vec_approx_eq!(actual.dir, expected.dir);
    }

    #[test]
    fn test_at() {
        let ray = Ray::new(Point::zero(), Vec3::new(-1.0, 1.0, -5.0));
        let actual = ray.at(42.0);
        let expected = Point::new(-42.0, 42.0, -210.0);
        assert_point_approx_eq!(actual, expected);
    }
}
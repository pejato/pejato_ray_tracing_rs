#[cfg(test)]
use super::*;
#[cfg(test)]
use crate::assert_point_approx_eq;

#[cfg(test)]
mod method_tests {
    use super::*;

    #[test]
    fn test_new() {
        let actual = Point::new(2.0, 4.0, 6.0);
        let expected = Point(Vec3::new(2.0, 4.0, 6.0));
        assert_point_approx_eq!(actual, expected);
    }

    #[test]
    fn test_zero() {
        let actual = Point::zero();
        let expected = Point(Vec3::zero());
        assert_point_approx_eq!(actual, expected);
    }
}

#[cfg(test)]
mod operator_tests {
    use super::*;

    #[test]
    fn test_plus() {
        let (lhs, rhs) = (Point::new(1.0, 2.0, 3.0), Vec3::new(-1.0, -2.0, -3.0));
        let actual = lhs + rhs.into();
        let expected = Point::new(0.0, 0.0, 0.0);
        assert_point_approx_eq!(actual, expected);
    }

    #[test]
    fn test_plus_equals() {
        let (mut lhs, rhs) = (Point::new(1.0, 2.0, 3.0), Vec3::new(-1.0, -2.0, -3.0));
        lhs += rhs.into();
        let actual = lhs;
        let expected = Point::new(0.0, 0.0, 0.0);
        assert_point_approx_eq!(actual, expected);
    }

    #[test]
    fn test_minus() {
        let (lhs, rhs) = (Point::new(1.0, 2.0, 3.0), Vec3::new(-1.0, -2.0, -3.0));
        let actual = lhs - rhs.into();
        let expected = Point::new(2.0, 4.0, 6.0);
        assert_point_approx_eq!(actual, expected);
    }

    #[test]
    fn test_minus_equals() {
        let (mut lhs, rhs) = (Point::new(1.0, 2.0, 3.0), Vec3::new(-1.0, -2.0, -3.0));
        lhs -= rhs.into();
        let actual = lhs;
        let expected = Point::new(2.0, 4.0, 6.0);
        assert_point_approx_eq!(actual, expected);
    }

    #[test]
    fn test_scalar_mult() {
        let (lhs, rhs) = (Point::new(1.0, 2.0, 3.0), 3.0);
        let actual = lhs * rhs;
        let expected = Point::new(3.0, 6.0, 9.0);
        assert_point_approx_eq!(actual, expected);
    }

    #[test]
    fn test_scalar_mult_equals() {
        let (mut lhs, rhs) = (Point::new(1.0, 2.0, 3.0), 3.0);
        lhs *= rhs;
        let actual = lhs;
        let expected = Point::new(3.0, 6.0, 9.0);
        assert_point_approx_eq!(actual, expected);
    }

    #[test]
    fn test_scalar_div() {
        let (lhs, rhs) = (Point::new(1.0, 2.0, 3.0), 3.0);
        let actual = lhs / rhs;
        let expected = Point::new(1.0 / 3.0, 2.0 / 3.0, 1.0);
        assert_point_approx_eq!(actual, expected);
    }

    #[test]
    fn test_scalar_div_equals() {
        let (mut lhs, rhs) = (Point::new(1.0, 2.0, 3.0), 3.0);
        lhs /= rhs;
        let actual = lhs;
        let expected = Point::new(1.0 / 3.0, 2.0 / 3.0, 1.0);
        assert_point_approx_eq!(actual, expected);
    }
}

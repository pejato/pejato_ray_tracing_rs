#[cfg(test)]
use super::{Color, Vec3};
#[cfg(test)]
use crate::assert_vec_approx_eq;

#[cfg(test)]
mod display_tests {
    use super::super::*;

    #[test]
    fn test_display_black() {
        let black = Color::new(0.0, 0.0, 0.0);
        let actual = format!("{black}");
        let expected = "0 0 0";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_display_white() {
        let white = Color::new(1.0, 1.0, 1.0);
        let actual = format!("{white}");
        let expected = "256 256 256";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_display_chartreuse() {
        let chartreuse = Color::new(0.5, 1.0, 0.0);
        let actual = format!("{chartreuse}");
        let expected = "128 256 0";
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod operator_tests {
    use super::*;

    #[test]
    fn test_plus() {
        let (lhs, rhs) = (Color::new(1.0, 2.0, 3.0), Vec3::new(-1.0, -2.0, -3.0));
        let actual = lhs + rhs.into();
        let expected = Color::new(0.0, 0.0, 0.0);
        assert_vec_approx_eq!(actual, expected);
    }

    #[test]
    fn test_plus_equals() {
        let (mut lhs, rhs) = (Color::new(1.0, 2.0, 3.0), Vec3::new(-1.0, -2.0, -3.0));
        lhs += rhs.into();
        let actual = lhs;
        let expected = Color::new(0.0, 0.0, 0.0);
        assert_vec_approx_eq!(actual, expected);
    }

    #[test]
    fn test_minus() {
        let (lhs, rhs) = (Color::new(1.0, 2.0, 3.0), Vec3::new(-1.0, -2.0, -3.0));
        let actual = lhs - rhs.into();
        let expected = Color::new(2.0, 4.0, 6.0);
        assert_vec_approx_eq!(actual, expected);
    }

    #[test]
    fn test_minus_equals() {
        let (mut lhs, rhs) = (Color::new(1.0, 2.0, 3.0), Vec3::new(-1.0, -2.0, -3.0));
        lhs -= rhs.into();
        let actual = lhs;
        let expected = Color::new(2.0, 4.0, 6.0);
        assert_vec_approx_eq!(actual, expected);
    }

    #[test]
    fn test_scalar_mult_left() {
        let (lhs, rhs) = (Color::new(1.0, 2.0, 3.0), 3.0);
        let actual = lhs * rhs;
        let expected = Color::new(3.0, 6.0, 9.0);
        assert_vec_approx_eq!(actual, expected);
    }

    #[test]
    fn test_scalar_mult_right() {
        let (lhs, rhs) = (Color::new(1.0, 2.0, 3.0), 3.0);
        let actual = rhs * lhs;
        let expected = Color::new(3.0, 6.0, 9.0);
        assert_vec_approx_eq!(actual, expected);
    }

    #[test]
    fn test_scalar_mult_equals() {
        let (mut lhs, rhs) = (Color::new(1.0, 2.0, 3.0), 3.0);
        lhs *= rhs;
        let actual = lhs;
        let expected = Color::new(3.0, 6.0, 9.0);
        assert_vec_approx_eq!(actual, expected);
    }

    #[test]
    fn test_scalar_div_left() {
        let (lhs, rhs) = (Color::new(1.0, 2.0, 3.0), 3.0);
        let actual = lhs / rhs;
        let expected = Color::new(1.0 / 3.0, 2.0 / 3.0, 1.0);
        assert_vec_approx_eq!(actual, expected);
    }

    #[test]
    fn test_scalar_div_equals() {
        let (mut lhs, rhs) = (Color::new(1.0, 2.0, 3.0), 3.0);
        lhs /= rhs;
        let actual = lhs;
        let expected = Color::new(1.0 / 3.0, 2.0 / 3.0, 1.0);
        assert_vec_approx_eq!(actual, expected);
    }
}

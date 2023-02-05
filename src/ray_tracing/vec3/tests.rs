#[cfg(test)]
use super::Vec3;
#[cfg(test)]
use assert_approx_eq::*;

#[cfg(test)]
macro_rules! assert_vec_approx_eq {
    ($lhs: ident, $rhs: ident) => {
        use assert_approx_eq::assert_approx_eq;

        assert_approx_eq!($lhs.a, $rhs.a);
        assert_approx_eq!($lhs.b, $rhs.b);
        assert_approx_eq!($lhs.c, $rhs.c);
    };
}

// MARK: - Methods

#[cfg(test)]
mod method_tests {
    use super::*;

    #[test]
    fn test_new() {
        let expected = Vec3 {
            a: 1.0,
            b: 2.0,
            c: 3.0,
        };
        let actual = Vec3::new(1.0, 2.0, 3.0);
        assert_vec_approx_eq!(actual, expected);
    }

    #[test]
    fn test_magnitude_zero_vector() {
        let expected = 0.0;
        let vec = Vec3::new(0.0, 0.0, 0.0);
        let actual = vec.magnitude();
        assert_approx_eq!(actual, expected);
    }

    #[test]
    fn test_magnitude_positive_vector() {
        let expected = f32::sqrt(14.0);
        let vec = Vec3::new(1.0, 2.0, 3.0);
        let actual = vec.magnitude();
        assert_approx_eq!(actual, expected);
    }

    #[test]
    fn test_magnitude_negative_vector() {
        let expected = f32::sqrt(14.0);
        let vec = Vec3::new(-1.0, -2.0, -3.0);
        let actual = vec.magnitude();
        assert_approx_eq!(actual, expected);
    }

    #[test]
    fn test_magnitude_mixed_vector() {
        let expected = f32::sqrt(14.0);
        let vec = Vec3::new(1.0, -2.0, 3.0);
        let actual = vec.magnitude();
        assert_approx_eq!(actual, expected);
    }

    #[test]
    fn test_summed() {
        let expected = 2.0;
        let vec = Vec3::new(1.0, -2.0, 3.0);
        let actual = vec.summed();
        assert_approx_eq!(actual, expected);
    }

    #[test]
    fn test_mul_elts() {
        let expected = Vec3::new(1.0 * 2.0, 3.0 * 4.0, 5.0 * 6.0);
        let lhs = Vec3::new(1.0, 3.0, 5.0);
        let rhs = Vec3::new(2.0, 4.0, 6.0);
        let actual = lhs.mul_elts(rhs);
        assert_vec_approx_eq!(actual, expected);
    }

    #[test]
    fn test_zero() {
        let expected = Vec3::new(0.0, 0.0, 0.0);
        let actual = Vec3::zero();
        assert_approx_eq!(actual.a, expected.a);
    }

    #[test]
    fn test_dot() {
        let expected = 1.0 * 2.0 + 3.0 * 4.0 + 5.0 * 6.0;
        let lhs = Vec3::new(1.0, 3.0, 5.0);
        let rhs = Vec3::new(2.0, 4.0, 6.0);
        let actual = lhs.dot(rhs);
        assert_approx_eq!(actual, expected);
    }
}

// MARK: - Display Impl

mod display_tests {
    use std::fmt::{format, Debug};

    use super::*;

    #[test]
    fn test_display() {
        let vec = Vec3::new(1.251, 4.421, 3.242);
        let actual = format!("{}", vec);
        let expected = "[1.251, 4.421, 3.242]";
        assert_eq!(actual, expected);
    }
}

// MARK: - Operators

#[cfg(test)]
mod operator_tests {
    use super::*;

    #[test]
    fn test_plus() {
        let (lhs, rhs) = (Vec3::new(1.0, 2.0, 3.0), Vec3::new(-1.0, -2.0, -3.0));
        let actual = lhs + rhs;
        let expected = Vec3::new(0.0, 0.0, 0.0);
        assert_vec_approx_eq!(actual, expected);
    }

    #[test]
    fn test_plus_equals() {
        let (mut lhs, rhs) = (Vec3::new(1.0, 2.0, 3.0), Vec3::new(-1.0, -2.0, -3.0));
        lhs += rhs;
        let actual = lhs;
        let expected = Vec3::new(0.0, 0.0, 0.0);
        assert_vec_approx_eq!(actual, expected);
    }

    #[test]
    fn test_minus() {
        let (lhs, rhs) = (Vec3::new(1.0, 2.0, 3.0), Vec3::new(-1.0, -2.0, -3.0));
        let actual = lhs - rhs;
        let expected = Vec3::new(2.0, 4.0, 6.0);
        assert_vec_approx_eq!(actual, expected);
    }

    #[test]
    fn test_minus_equals() {
        let (mut lhs, rhs) = (Vec3::new(1.0, 2.0, 3.0), Vec3::new(-1.0, -2.0, -3.0));
        lhs -= rhs;
        let actual = lhs;
        let expected = Vec3::new(2.0, 4.0, 6.0);
        assert_vec_approx_eq!(actual, expected);
    }

    #[test]
    fn test_scalar_mult() {
        let (lhs, rhs) = (Vec3::new(1.0, 2.0, 3.0), 3.0);
        let actual = lhs * rhs;
        let expected = Vec3::new(3.0, 6.0, 9.0);
        assert_vec_approx_eq!(actual, expected);
    }

    #[test]
    fn test_scalar_mult_equals() {
        let (mut lhs, rhs) = (Vec3::new(1.0, 2.0, 3.0), 3.0);
        lhs *= rhs;
        let actual = lhs;
        let expected = Vec3::new(3.0, 6.0, 9.0);
        assert_vec_approx_eq!(actual, expected);
    }

    #[test]
    fn test_scalar_div() {
        let (lhs, rhs) = (Vec3::new(1.0, 2.0, 3.0), 3.0);
        let actual = lhs / rhs;
        let expected = Vec3::new(1.0 / 3.0, 2.0 / 3.0, 1.0);
        assert_vec_approx_eq!(actual, expected);
    }

    #[test]
    fn test_scalar_div_equals() {
        let (mut lhs, rhs) = (Vec3::new(1.0, 2.0, 3.0), 3.0);
        lhs /= rhs;
        let actual = lhs;
        let expected = Vec3::new(1.0 / 3.0, 2.0 / 3.0, 1.0);
        assert_vec_approx_eq!(actual, expected);
    }
}

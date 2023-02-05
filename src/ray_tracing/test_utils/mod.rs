#[cfg(test)]
#[macro_export]
macro_rules! assert_vec_approx_eq {
    ($lhs: expr, $rhs: expr) => {
        assert_approx_eq::assert_approx_eq!($lhs.a, $rhs.a);
        assert_approx_eq::assert_approx_eq!($lhs.b, $rhs.b);
        assert_approx_eq::assert_approx_eq!($lhs.c, $rhs.c);
    };
}

#[cfg(test)]
#[macro_export]
macro_rules! assert_point_approx_eq {
    ($lhs: expr, $rhs: expr) => {
        let lhs_as_vec: Vec3 = $lhs.into();
        let rhs_as_vec: Vec3 = $rhs.into();
        assert_approx_eq::assert_approx_eq!(lhs_as_vec.a, rhs_as_vec.a);
        assert_approx_eq::assert_approx_eq!(lhs_as_vec.b, rhs_as_vec.b);
        assert_approx_eq::assert_approx_eq!(lhs_as_vec.c, rhs_as_vec.c);
    };
}

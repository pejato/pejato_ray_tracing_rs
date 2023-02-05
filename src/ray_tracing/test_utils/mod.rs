#[cfg(test)]
#[macro_export]
macro_rules! assert_vec_approx_eq {
    ($lhs: expr, $rhs: expr) => {
        let lhs_as_vec: $crate::ray_tracing::vec3::Vec3 = $lhs.into(); 
        let rhs_as_vec: $crate::ray_tracing::vec3::Vec3 = $rhs.into(); 
        assert_approx_eq::assert_approx_eq!(lhs_as_vec.x, rhs_as_vec.x);
        assert_approx_eq::assert_approx_eq!(lhs_as_vec.y, rhs_as_vec.y);
        assert_approx_eq::assert_approx_eq!(lhs_as_vec.z, rhs_as_vec.z);
    };
}
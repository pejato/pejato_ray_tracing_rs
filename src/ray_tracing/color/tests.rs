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

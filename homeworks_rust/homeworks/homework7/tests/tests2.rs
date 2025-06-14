// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        assert_eq!( 3 * 1, 3);
        assert_eq!(3, 3 * 1);
    }

    #[test]
    fn you_can_assert_eq1() {
        assert_eq!(3 * 1, 255);
    }
/* This test would cause error[E0308]: mismatched types and error[E0277]: can't compare `{integer}` with `bool`
    #[test]
    fn you_can_assert_eq2() {
        assert_eq!(3 *1, true);
    } */
}

//SOLution
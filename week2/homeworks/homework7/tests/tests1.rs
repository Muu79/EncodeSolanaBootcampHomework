// tests1.rs
// Tests are important to ensure that your code does what you think it should do.
// Tests can be run on this file with the following command:
// rustlings run tests1

// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests1` for hints :)


#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert() {
        let x = 5;
        assert!(8 * 2_i32.pow(5) == 8 << 5);
        assert_eq!(0b1 << 1, 0b10);
        assert_eq!(0b1010 ^ 15, 0b101);
    }
}

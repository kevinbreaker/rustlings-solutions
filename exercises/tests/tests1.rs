// tests1.rs
// Tests are important to ensure that your code does what you think it should do.
// Tests can be run on this file with the following command:
// rustlings run tests1

// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests1` for hints :)


fn multiplier(numberA: i32, numberB: i32) -> i32 {
    numberA * numberB
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplier() {
        const RESULT: i32 = 12 * 24;
        let multiplier = multiplier(12, 24);

        assert!(multiplier == RESULT);
    }
}

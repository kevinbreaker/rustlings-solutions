// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)

fn multiplier(numberA: i32, numberB: i32) -> i32 {
    numberA * numberB
}

#[cfg(test)]
mod tests {
    use super::multiplier;

    #[test]
    fn you_can_assert_eq() {
        const RESULT:i32 = 34 * 86;

        assert_eq!(multiplier(34, 86), RESULT);
    }
}

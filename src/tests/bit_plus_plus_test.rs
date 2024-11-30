#[cfg(test)]
mod tests {
    use crate::problems::bit_plus_plus::testing_main;

    #[test]
    fn test_base_case() {
        let input: &[&str] = &["1", "++x"];
        let result: String = testing_main(input);
        let expected: usize = 1;
        assert_eq!(result, expected.to_string())
    }

    #[test]
    fn test_scenario_second() {
        let input: &[&str] = &["2", "x++", "--x"];
        let result: String = testing_main(input);
        let expected: usize = 0;
        assert_eq!(result, expected.to_string());
    }
}

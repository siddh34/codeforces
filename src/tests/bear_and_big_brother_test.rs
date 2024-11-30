#[cfg(test)]
mod tests {
    use crate::problems::bear_and_big_brother::testing_main;

    #[test]
    fn test_base_case() {
        let input: &str = "4 7";
        let result: String = testing_main(input);
        let expected: usize = 2;
        assert_eq!(result, expected.to_string())
    }

    #[test]
    fn test_scenario_second() {
        let input: &str = "4 9";
        let result: String = testing_main(input);
        let expected: usize = 3;
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_scenario_third() {
        let input: &str = "1 1";
        let result: String = testing_main(input);
        let expected: usize = 1;
        assert_eq!(result, expected.to_string());
    }
}

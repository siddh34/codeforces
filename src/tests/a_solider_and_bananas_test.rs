#[cfg(test)]
mod tests {
    use crate::problems::a_solider_and_bananas::testing_main;

    #[test]
    fn test_base_case() {
        let input: &str = "3 17 4";
        let result: String = testing_main(input);
        let expected: usize = 13;
        assert_eq!(result, expected.to_string())
    }

    #[test]
    fn test_scenario_second() {
        let input: &str = "5 20 9";
        let result: String = testing_main(input);
        let expected: usize = 205;
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_scenario_third() {
        let input: &str = "6 100 10";
        let result: String = testing_main(input);
        let expected: usize = 230;
        assert_eq!(result, expected.to_string());
    }
}
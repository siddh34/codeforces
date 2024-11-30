#[cfg(test)]

mod tests {
    use crate::problems::beautiful_matrix::testing_main;

    #[test]
    fn test_base_case() {
        let input: &str = "0 0 0 0 0\n0 0 0 0 1\n0 0 0 0 0\n0 0 0 0 0\n0 0 0 0 0";
        let result: String = testing_main(input);
        let expected: usize = 3;
        assert_eq!(result, expected.to_string())
    }

    #[test]
    fn test_scenario_second() {
        let input: &str = "0 0 0 0 0\n0 0 0 0 0\n0 1 0 0 0\n0 0 0 0 0\n0 0 0 0 0";
        let result: String = testing_main(input);
        let expected: usize = 1;
        assert_eq!(result, expected.to_string());
    }
}

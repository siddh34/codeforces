#[cfg(test)]
mod tests {
    use crate::problems::petya_and_strings::testing_main;

    #[test]
    fn test_base_case() {
        let input: &str = "aaaa\naaaA";
        let result: String = testing_main(input);
        let expected: isize = 0;
        assert_eq!(result, expected.to_string())
    }

    #[test]
    fn test_scenario_second(){
        let input: &str = "abs\nAbz";
        let result: String = testing_main(input);
        let expected: isize = -1;
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_scenario_third(){
        let input: &str = "abcdefg\nAbCdEfF";
        let result: String = testing_main(input);
        let expected: isize = 1;
        assert_eq!(result, expected.to_string());
    }
}
#[cfg(test)]
mod tests {
    use crate::problems::word_capitalization::testing_main;

    #[test]
    fn test_base_case() {
        let input: &str = "konjac";
        let result: String = testing_main(input);
        assert_eq!(result, "Konjac");
    }

    #[test]
    fn test_scenario_second() {
        let input: &str = "KONJAC";
        let result: String = testing_main(input);
        assert_eq!(result, "KONJAC");
    }
}
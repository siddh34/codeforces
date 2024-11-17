#[cfg(test)]
mod tests {
    use crate::problems::boy_or_girl::testing_main;

    #[test]
    fn test_base_case() {
        let input: &str = "wjmzbmr";
        let result: String = testing_main(input);
        assert_eq!(result, "CHAT WITH HER!");
    }

    #[test]
    fn test_scenario_second() {
        let input: &str = "sevenkplus";
        let result: String = testing_main(input);
        assert_eq!(result, "CHAT WITH HER!");
    }

    #[test]
    fn test_scenario_third() {
        let input: &str = "xiaodao";
        let result: String = testing_main(input);
        assert_eq!(result, "IGNORE HIM!");
    }
}

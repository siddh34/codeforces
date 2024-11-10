#[cfg(test)]
mod tests {
    use crate::problems::helpful_maths::testing_main;

    #[test]
    fn test_base_case() {
        let input: &str = "1+1+3+1+3";
        let result: String = testing_main(input);
        assert_eq!(result, "1+1+1+3+3");
    }

    #[test]
    fn test_scenario_second() {
        let input: &str = "2+2+1+3+1";
        let result: String = testing_main(input);
        assert_eq!(result, "1+1+2+2+3");
    }
}

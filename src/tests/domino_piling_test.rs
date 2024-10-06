#[cfg(test)]
mod tests {
    use crate::problems::domino_piling::testing_main;

    #[test]
    fn test_base_case(){
        let input: &str = "2 4";
        let result: String = testing_main(input);
        assert_eq!(result, "4");
    }

    #[test]
    fn test_scenario_second(){
        let input: &str = "3 3";
        let result: String = testing_main(input);
        assert_eq!(result, "4");
    }
}
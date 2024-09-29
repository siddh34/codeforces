#[cfg(test)]
mod tests {
    use crate::problems::theatre_square::testing_main;

    #[test]
    fn test_theatre_square_base_case() {
        let input = "6 6 4";
        let expected = "4"; // Corrected expected output
        let output: String = testing_main(input);
        assert_eq!(output, expected);
    }
}

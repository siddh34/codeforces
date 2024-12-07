#[cfg(test)]
mod tests {
    use crate::problems::stones_on_the_table::testing_main;

    #[test]
    fn test_1() {
        let input = "3\nRRG\n";
        let output = "1";
        assert_eq!(testing_main(input), output);
    }

    #[test]
    fn test_2() {
        let input = "5\nRRRRR\n";
        let output = "4";
        assert_eq!(testing_main(input), output);
    }

    #[test]
    fn test_3() {
        let input = "4\nBRBG\n";
        let output = "0";
        assert_eq!(testing_main(input), output);
    }

    #[test]
    fn test_4() {
        let input = "1\nR\n";
        let output = "0";
        assert_eq!(testing_main(input), output);
    }

    #[test]
    fn test_5() {
        let input = "1\nG\n";
        let output = "0";
        assert_eq!(testing_main(input), output);
    }
}
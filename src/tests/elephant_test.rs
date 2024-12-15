#[cfg(test)]
mod tests {
    use crate::problems::elephant::testing_main;

    #[test]
    fn test_1() {
        assert_eq!(testing_main("5"), "1");
    }

    #[test]
    fn test_2() {
        assert_eq!(testing_main("12"), "3");
    }

    #[test]
    fn test_3() {
        assert_eq!(testing_main("1"), "1");
    }

    #[test]
    fn test_4() {
        assert_eq!(testing_main("10"), "2");
    }

    #[test]
    fn test_5() {
        assert_eq!(testing_main("100"), "20");
    }
}
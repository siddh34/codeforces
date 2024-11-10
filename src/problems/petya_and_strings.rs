use std::io::{self};

/*
A. Petya and Strings
time limit per test2 seconds
memory limit per test256 megabytes
Little Petya loves presents. His mum bought him two strings of the same size for his birthday. The strings consist of uppercase and lowercase Latin letters. Now Petya wants to compare those two strings lexicographically. The letters' case does not matter, that is an uppercase letter is considered equivalent to the corresponding lowercase letter. Help Petya perform the comparison.

Input
Each of the first two lines contains a bought string. The strings' lengths range from 1 to 100 inclusive. It is guaranteed that the strings are of the same length and also consist of uppercase and lowercase Latin letters.

Output
If the first string is less than the second one, print "-1". If the second string is less than the first one, print "1". If the strings are equal, print "0". Note that the letters' case is not taken into consideration when the strings are compared.

Examples
Input
aaaa
aaaA
Output
0

Input
abs
Abz
Output
-1

Input
abcdefg
AbCdEfF
Output
1

Note
If you want more formal information about the lexicographical order (also known as the "dictionary order" or "alphabetical order"), you can visit the following site:
*/

pub fn testing_main(input: &str) -> String {
    let mut str_arr = input.trim().split("\n");

    let str1: &str = str_arr.next().unwrap();
    let str2: &str = str_arr.next().unwrap();

    let result: std::cmp::Ordering = str1.to_lowercase().cmp(&str2.to_lowercase());

    match result {
        std::cmp::Ordering::Less => "-1".to_string(),
        std::cmp::Ordering::Equal => "0".to_string(),
        std::cmp::Ordering::Greater => "1".to_string(),
    }
}

pub fn main() {
    let mut str1: String = String::new();
    let mut str2: String = String::new();

    io::stdin().read_line(&mut str1).expect("Failed to read str1");
    io::stdin().read_line(&mut str2).expect("Failed to read str2");

    let result: std::cmp::Ordering = str1.trim().to_lowercase().cmp(&str2.trim().to_lowercase());

    match result {
        std::cmp::Ordering::Less => println!("-1"),
        std::cmp::Ordering::Equal => println!("0"),
        std::cmp::Ordering::Greater => println!("1"),
    }
}
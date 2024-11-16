use std::io::{self};

/*
281A. Word Capitalization
Capitalization is writing a word with its first letter as a capital letter. Your task is to capitalize the given word.

Note, that during capitalization all the letters except the first one remains unchanged.

Input
A single line contains a non-empty word. This word consists of lowercase and uppercase English letters. The length of the word will not exceed 103.

Output
Output the given word after capitalization.

Examples
Input
ApPLe
Output
ApPLe
Input
konjac
Output
Konjac

*/

pub fn testing_main(input_str: &str) -> String {
    let word: String = input_str.trim().to_string();
    let first_letter: char = word.chars().nth(0).unwrap();
    let rest_of_word: &str = &word[1..];

    return format!("{}{}", first_letter.to_uppercase(), rest_of_word);
}

pub fn main() {
    let mut input_line: String = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .map_err(|_| "Failed to read input!")
        .unwrap();

    let word: String = input_line.trim().to_string();
    let first_letter: char = word.chars().nth(0).unwrap();
    let rest_of_word: &str = &word[1..];

    println!("{}{}", first_letter.to_uppercase(), rest_of_word);
}

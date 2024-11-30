use std::io::{self};

/*
Problem Statement 71A - Way Too Long Words

Sometimes some words like "localization" or "internationalization" are so long that writing them many times in one text is quite tiresome.

Let's consider a word too long, if its length is strictly more than 10 characters. All too long words should be replaced with a special abbreviation.

This abbreviation is made like this: we write down the first and the last letter of a word and between them we write the number of letters between the first and the last letters. That number is in decimal system and doesn't contain any leading zeroes.

Thus, "localization" will be spelt as "l10n", and "internationalization» will be spelt as "i18n".

You are suggested to automatize the process of changing the words with abbreviations. At that all too long words should be replaced by the abbreviation and the words that are not too long should not undergo any changes.

Input
The first line contains an integer n (1 ≤ n ≤ 100). Each of the following n lines contains one word. All the words consist of lowercase Latin letters and possess the lengths of from 1 to 100 characters.

Output
Print n lines. The i-th line should contain the result of replacing of the i-th word from the input data.
*/

pub fn main() {
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read starting number!");

    let number: i32 = input_line.trim().parse().expect("Failed to parse input");

    for _ in 0..number {
        let mut input_line = String::new();
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read inside loop!");
        let word: String = input_line.trim().to_string();

        if word.len() > 10 {
            let first_letter = word.chars().nth(0).unwrap();
            let last_letter = word.chars().nth(word.len() - 1).unwrap();
            let mid_length = word.len() - 2;
            println!("{}{}{}", first_letter, mid_length, last_letter);
        } else {
            println!("{}", word);
        }
    }
}

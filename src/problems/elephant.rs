use std::io::{self};

/*
A. Elephant
time limit per test1 second
memory limit per test256 megabytes
An elephant decided to visit his friend. It turned out that the elephant's house is located at point 0 and his friend's house is located at point x(x > 0) of the coordinate line. In one step the elephant can move 1, 2, 3, 4 or 5 positions forward. Determine, what is the minimum number of steps he need to make in order to get to his friend's house.

Input
The first line of the input contains an integer x (1 ≤ x ≤ 1 000 000) — The coordinate of the friend's house.

Output
Print the minimum number of steps that elephant needs to make to get from point 0 to point x.

Examples
InputCopy
5
OutputCopy
1
InputCopy
12
OutputCopy
3
*/

pub fn testing_main(input: &str) -> String {
    let number: i32 = input.trim().parse().unwrap();
    let result: i32 = number / 5 + if number % 5 == 0 { 0 } else { 1 };
    return result.to_string();
}

pub fn main() {
    let mut input_line: String = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .map_err(|err| {
            eprintln!("Error: {}", err);
        })
        .ok();

    let number: i32 = input_line.trim().parse().unwrap();
    let result: i32 = number / 5 + if number % 5 == 0 { 0 } else { 1 };
    println!("{}", result);
}

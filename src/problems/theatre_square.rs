use std::io::{self};

/*
A. Theatre Square
Theatre Square in the capital city of Berland has a rectangular shape with the size n × m meters. On the occasion of the city's anniversary, a decision was taken to pave the Square with square granite flagstones. Each flagstone is of the size a × a.

What is the least number of flagstones needed to pave the Square? It's allowed to cover the surface larger than the Theatre Square, but the Square has to be covered. It's not allowed to break the flagstones. The sides of flagstones should be parallel to the sides of the Square.

Input
The input contains three positive integer numbers in the first line: n,  m and a (1 ≤  n, m, a ≤ 109).

Output
Write the needed number of flagstones.

Input
6 6 4
Output
4
*/

pub fn testing_main(input: &str) -> String {
    let mut numbers = input.trim().split_whitespace();
    let n: u64 = numbers.next().unwrap().parse().unwrap();
    let m: u64 = numbers.next().unwrap().parse().unwrap();
    let a: u64 = numbers.next().unwrap().parse().unwrap();

    let result = ((n + a - 1) / a) * ((m + a - 1) / a);
    return result.to_string()
}

pub fn main() {
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read starting numbers!");

    let mut numbers = input_line.trim().split_whitespace();
    let n: u64 = numbers.next().unwrap().parse().unwrap();
    let m: u64 = numbers.next().unwrap().parse().unwrap();
    let a: u64 = numbers.next().unwrap().parse().unwrap();

    let result = ((n + a - 1) / a) * ((m + a - 1) / a);
    println!("{}", result);
}



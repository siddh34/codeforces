use std::io::{self};

/*
Problem Statement 231A - Team

One day three best friends Petya, Vasya and Tonya decided to form a team and take part in programming contests. Participants are usually offered several problems during programming contests. Long before the start the friends decided that they will implement a problem if at least two of them are sure about the solution. Otherwise, the friends won't write the problem's solution.

This contest offers n problems to the participants. For each problem we know, which friend is sure about the solution. Help the friends find the number of problems for which they will write a solution.

Input
The first input line contains a single integer n (1 ≤ n ≤ 1000) — the number of problems in the contest. Then n lines contain three integers each, each integer is either 0 or 1. If the first number in the line equals 1, then Petya is sure about the problem's solution, otherwise he isn't sure. The second number shows Vasya's view on the solution, the third number shows Tonya's view. The numbers on the lines are separated by spaces.

Output
Print a single integer — the number of problems the friends will implement on the contest.
*/

pub fn main() {
    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    let number: i32 = input_string.trim().parse().expect("Failed to parse input");
    let mut solved_prb: i32 = 0;
    for _ in 0..number {
        let mut input_string = String::new();
        io::stdin()
            .read_line(&mut input_string)
            .expect("Failed to read line");
        let mut problem = input_string
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap());
        let petya = problem.next().unwrap();
        let vasya = problem.next().unwrap();
        let tonya = problem.next().unwrap();
        if petya + vasya + tonya >= 2 {
            solved_prb += 1;
        }
    }
    println!("{}", solved_prb);
}

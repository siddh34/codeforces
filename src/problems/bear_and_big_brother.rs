use std::io::{self};

/*
A. Bear and Big Brother
time limit per test1 second
memory limit per test256 megabytes
Bear Limak wants to become the largest of bears, or at least to become larger than his brother Bob.

Right now, Limak and Bob weigh a and b respectively. It's guaranteed that Limak's weight is smaller than or equal to his brother's weight.

Limak eats a lot and his weight is tripled after every year, while Bob's weight is doubled after every year.

After how many full years will Limak become strictly larger (strictly heavier) than Bob?

Input
The only line of the input contains two integers a and b (1 ≤ a ≤ b ≤ 10) — the weight of Limak and the weight of Bob respectively.

Output
Print one integer, denoting the integer number of years after which Limak will become strictly larger than Bob.

Examples
Input
4 7
Output
2
Input
4 9
Output
3
Input
1 1
Output
1
*/

pub fn testing_main(input_line: &str) -> String {
    let mut input_iter = input_line.trim().split_whitespace();

    let mut limak_weight: i32 = input_iter.next().unwrap().parse().unwrap();
    let mut bob_weight: i32 = input_iter.next().unwrap().parse().unwrap();
    let mut years: i32 = 0;
    while limak_weight <= bob_weight {
        limak_weight *= 3;
        bob_weight *= 2;
        years += 1;
    }

    years.to_string()
}

pub fn main() {
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .map_err(|err| {
            eprintln!("Error: {}", err);
        })
        .ok();
    let mut input_iter = input_line.trim().split_whitespace();

    let mut limak_weight: i32 = input_iter.next().unwrap().parse().unwrap();
    let mut bob_weight: i32 = input_iter.next().unwrap().parse().unwrap();
    let mut years: i32 = 0;
    while limak_weight <= bob_weight {
        limak_weight *= 3;
        bob_weight *= 2;
        years += 1;
    }

    println!("{}", years);
}

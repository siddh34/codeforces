use std::io;

/*
A. Soldier and Bananas
time limit per test1 second
memory limit per test256 megabytes
A soldier wants to buy w bananas in the shop. He has to pay k dollars for the first banana, 2k dollars for the second one and so on (in other words, he has to pay i·k dollars for the i-th banana).

He has n dollars. How many dollars does he have to borrow from his friend soldier to buy w bananas?

Input
The first line contains three positive integers k, n, w (1  ≤  k, w  ≤  1000, 0 ≤ n ≤ 109), the cost of the first banana, initial number of dollars the soldier has and number of bananas he wants.

Output
Output one integer — the amount of dollars that the soldier must borrow from his friend. If he doesn't have to borrow money, output 0.

Examples
InputCopy
3 17 4
OutputCopy
13
*/

pub fn testing_main(input_str: &str) -> String {
    let mut input = input_str.split_whitespace();
    let k: i32 = input.next().unwrap().parse().unwrap();
    let n: i32 = input.next().unwrap().parse().unwrap();
    let w: i32 = input.next().unwrap().parse().unwrap();

    let mut total_cost: i32 = 0;

    for i in 1..=w {
        total_cost += i * k;
    }

    let borrow: i32 = total_cost - n;
    if borrow < 0 {
        return "0".to_string();
    }
    return borrow.to_string();
}

pub fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| "Failed to read line")
        .unwrap();

    let mut input = input.split_whitespace();
    let k: i32 = input.next().unwrap().parse().unwrap();
    let n: i32 = input.next().unwrap().parse().unwrap();
    let w: i32 = input.next().unwrap().parse().unwrap();

    let mut total_cost: i32 = 0;

    for i in 1..=w {
        total_cost += i * k;
    }

    let mut borrow: i32 = total_cost - n;
    if borrow < 0 {
        borrow = 0;
    }
    println!("{}", borrow);
}

use std::io::{self};

/*
Problem statement 158A - Next Round
"Contestant who earns a score equal to or greater than the k-th place finisher's score will advance to the next round, as long as the contestant earns a positive score..." — an excerpt from contest rules.

A total of n participants took part in the contest (n ≥ k), and you already know their scores. Calculate how many participants will advance to the next round.

Input
The first line of the input contains two integers n and k (1 ≤ k ≤ n ≤ 50) separated by a single space.

The second line contains n space-separated integers a1, a2, ..., an (0 ≤ ai ≤ 100), where ai is the score earned by the participant who got the i-th place. The given sequence is non-increasing (that is, for all i from 1 to n - 1 the following condition is fulfilled: ai ≥ ai + 1).

Output
Output the number of participants who advance to the next round.
*/

pub fn main() {
    let mut number_str = String::new();
    io::stdin()
        .read_line(&mut number_str)
        .expect("Failed to read the line");

    let mut number = number_str
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap());

    let _n = number.next().unwrap();
    let k = number.next().unwrap();

    let mut scores_str = String::new();
    io::stdin()
        .read_line(&mut scores_str)
        .expect("Failed to read the line");

    let scores = scores_str
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let k_score = scores[k as usize - 1];
    let mut count = 0;

    for score in scores {
        if score >= k_score && score > 0 {
            count += 1;
        } else {
            break;
        }
    }
    println!("{}", count);
}

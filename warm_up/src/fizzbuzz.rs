use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<i64> = line.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        let x = nums[0];
        let y = nums[1];
        let n = nums[2];

        fizz_buzz(x, y, n);
    }
}

fn fizz_buzz(x: i64, y: i64, n: i64) {
    for i in 1..=n {
        if i % x == 0 && i % y == 0 {
            println!("FizzBuzz");
        } else if i % x == 0 {
            println!("Fizz");
        } else if i % y == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}

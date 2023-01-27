use std::io::{self, BufRead};

pub fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    
    let n : usize = (iterator.next().unwrap().unwrap())
        .parse::<usize>().unwrap(); 

    let nums: Vec<i64> = iterator.next().unwrap().unwrap()
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    print_num_temps_below_0(n as usize, &nums);
}

fn print_num_temps_below_0(n: usize, temps: &[i64]) -> () {
    let mut x = 0;
    for t in 0..n {
        if temps[t] < 0 {
            x += 1;
        }
    }
    println!("{}", x);
}

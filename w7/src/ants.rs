use std::io::{self, Read};

pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let test_cases: i32 = input.next().unwrap().parse().unwrap(); 

    for _ in 0..test_cases {
        let pole_len: i32 = input.next().unwrap().parse().unwrap(); 
        let n_ants: i32 = input.next().unwrap().parse().unwrap();

        let mut ant: i32 = input.next().unwrap().parse().unwrap(); 
        let mut earliest: i32 = ant.min(pole_len - ant);
        let mut latest: i32 = earliest.max(pole_len - earliest);
        for _ in 1..n_ants {
            ant = input.next().unwrap().parse().unwrap(); 
            earliest = earliest.max(ant.min(pole_len - ant));
            latest = latest.max(ant.max(pole_len - ant));
        }
        println!("{earliest} {latest}");
    }
}
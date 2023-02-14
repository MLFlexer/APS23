use std::io::{self, Read};

pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    
    let n : i32 = input.next().unwrap().parse().unwrap();
    let mut a = 1;
    let mut b = 0;

    for _ in 1..=n {
        let tmp = a + b;
        a = b;
        b = tmp;
    }
    println!("{} {}", a, b)
}

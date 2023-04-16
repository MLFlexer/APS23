use std::io::{self, Read};

pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();
    let my_str: &str = input.next().unwrap();
    let his_str: &str = input.next().unwrap();
    let mut eq = 0;
    let mut my_iter = my_str.chars();
    let mut his_iter = his_str.chars();
    for i in 0..my_str.len() {
        let my_char = my_iter.next().unwrap();
        let his_char = his_iter.next().unwrap();
        if my_char == his_char {
            eq += 1;
        }
    }

    if eq > n {
        println!("{}", my_str.len() - eq + n);
    } else {
        println!("{}", my_str.len() + eq - n);
    }
}

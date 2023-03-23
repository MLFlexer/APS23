use std::io::{self, Read};

pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let h: u32 = input.next().unwrap().parse().unwrap();

    let num_nodes: i32 = 2_i32.pow(h + 1) - 1;
    match input.next() {
        Some(s) => {
            let mut i = 1;
            for c in s.chars() {
                match c {
                    'L' => {
                        i = 2 * i;
                    }
                    'R' => {
                        i = 2 * i + 1;
                    }
                    _ => {}
                }
            }
            println!("{}", num_nodes - i + 1);
        }
        None => {
            println!("{}", num_nodes);
        }
    }
}

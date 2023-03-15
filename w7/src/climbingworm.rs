use std::io::{self, Read};

pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let a: i32 = input.next().unwrap().parse().unwrap();
    let b: i32 = input.next().unwrap().parse().unwrap();
    let h: i32 = input.next().unwrap().parse().unwrap();

    let mut curr_height = 0;
    let mut count = 0;
    while curr_height < h {
        curr_height += a;
        count += 1;
        if curr_height >= h {
            println!("{}", count);
        } else {
            curr_height -= b;
        }
    }
}

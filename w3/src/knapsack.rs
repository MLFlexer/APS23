use std::{io::{self, Read}, collections::HashMap};

pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    loop {
        match input.next() {
            None => break,
            Some(s) => {
                let c : i32 = s.parse().unwrap();
                let n : i32 = input.next().unwrap().parse().unwrap();

                let mut items : Vec<(i32, i32, i32)> = vec![];

                for i in 0..n {
                    let v : i32 = input.next().unwrap().parse().unwrap();
                    let w : i32 = input.next().unwrap().parse().unwrap();

                    items.push((v, w, i));
                }

                items.sort_by(|(a_v, _, _), (b_v, _, _)| a_v.partial_cmp(b_v).unwrap());

                search();
            }
        }
    }
}

fn search(c : i32, items : Vec<(i32, i32, i32)>) -> Vec<i32> {
    return vec![];
}
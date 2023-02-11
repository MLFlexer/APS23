use std::io::{self, Read};

pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    
    let t : i32 = input.next().unwrap().parse().unwrap();

    for t_i in 0..t {

        let n : i32 = input.next().unwrap().parse().unwrap();
        let mut v1 : Vec<i64> = vec![];
        let mut v2 : Vec<i64> = vec![];
        for _ in 0..n {
            let x : i64 = input.next().unwrap().parse().unwrap();
            v1.push(x);
        }
        for _ in 0..n {
            let y : i64 = input.next().unwrap().parse().unwrap();
            v2.push(y);
        }
        v1.sort();
        v2.sort();
        v2.reverse();
        let mut v1_i = v1.iter();
        let mut v2_i = v2.iter();
        let mut scal : i64 = 0;
        for _ in 0..n {
            scal += v1_i.next().unwrap() * v2_i.next().unwrap();
        }
        println!("Case #{}: {}", t_i + 1, scal);
    }
}

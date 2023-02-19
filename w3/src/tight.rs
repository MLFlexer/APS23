use std::{io::{self, Read}, collections::HashMap};

pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    loop {
        match input.next() {
            None => break,
            Some(s) => {
                let k : i32 = s.parse().unwrap();
                let n : i32 = input.next().unwrap().parse().unwrap();

                let mut set : HashMap<(i32, i32), i128> = HashMap::new();
                let mut sum : i128 = 0;

                if n == 1  || k == 0 {
                    println!("{:.9}", 100.000000000);
                } else {
                    for i in 0..=k {
                        sum += search(1, i, n, k, &mut set);
                    }
                    println!("{:.9}", (sum as f64 / (((k + 1) as i128).pow((n as u32))) as f64) * 100.00 );
                }
            }
        }
    }
}

fn search(lvl : i32, prev_k : i32, n : i32, k : i32, set : &mut HashMap<(i32, i32), i128>) -> i128 {
    //base case
    if lvl == n - 1 {
        if prev_k == 0 || prev_k == k {
            return 2;
        } else {
            return 3;
        }
    } else {
        let mut sum : i128 = 0;

        match set.get(&(lvl, prev_k)) {
            Some(val) => {
                sum += val;
            },
            None => {
                let val = search(lvl + 1, prev_k, n, k, set);
                sum += val;
                if prev_k == 0 {
                    let val = search(lvl + 1, prev_k + 1, n, k, set);
                    sum += val;
                } else if prev_k == k {
                    let val = search(lvl + 1, prev_k - 1, n, k, set);
                    sum += val;
                } else {
                    let val = search(lvl + 1, prev_k + 1, n, k, set);
                    sum += val;
                    let val = search(lvl + 1, prev_k - 1, n, k, set);
                    sum += val;
                }
            },
        }
        set.insert((lvl, prev_k), sum);
        return sum;
    }
}
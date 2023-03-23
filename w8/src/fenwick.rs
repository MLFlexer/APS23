use std::io::{self, Read};

pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n: i64 = input.next().unwrap().parse().unwrap();
    let q: i64 = input.next().unwrap().parse().unwrap();

    let mut tree: Vec<i64> = vec![0; (n + 1) as usize];
    let mut s = String::with_capacity((n * 20) as usize);

    for _ in 0..q {
        match input.next().unwrap() {
            "+" => {
                let i: i64 = input.next().unwrap().parse().unwrap();
                let x: i64 = input.next().unwrap().parse().unwrap();
                add(i + 1, x, &mut tree, n);
            }
            "?" => {
                let i: i64 = input.next().unwrap().parse().unwrap();

                s.push_str(&format!("{} \n", sum(i, &mut tree)));
            }
            _ => {
                println!("err");
            }
        }
    }
    println!("{s}");
}

fn sum(k: i64, tree: &mut Vec<i64>) -> i64 {
    let mut k = k;
    let mut s = 0;
    while k >= 1 {
        s += tree[k as usize];
        k -= k & -k;
    }
    return s;
}

fn add(k: i64, x: i64, tree: &mut Vec<i64>, n: i64) -> () {
    let mut k = k;
    while k <= n {
        tree[k as usize] += x;
        k += k & -k;
    }
}

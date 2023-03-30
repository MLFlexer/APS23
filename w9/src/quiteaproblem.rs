use std::{
    ffi::c_void,
    io::{self, BufRead},
};

pub fn main() {
    let prob_str: &str = "problem";
    let a = 2;
    let b: u64 = 2_u64.pow(63) - 1;
    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let words: Vec<&str> = line.split_whitespace().collect();
        for word in words {
            if word.len() >= prob_str.len() {
                preprocessing(word, a, b);
            }
        }
    }
}

fn preprocessing(word: &str, a: u32, b: u64) {
    let n = word.len();
    let mut chars = word.chars();
    //TODO: Tjek om chars bliver lavet om korrekt
    let mut H: Vec<u32> = Vec::with_capacity(n);
    H.push(chars.next().unwrap().to_ascii_lowercase() as u32)

    let mut P: Vec<u32> = Vec::with_capacity(n);
    P.push(1);
    for i in 1..n {
        let c = chars.next().unwrap();
        println!("{}", c);
        let c_val = c.to_ascii_lowercase() as u32;
        H.push(c_val);
    }
    println!("{:?}", H);
}

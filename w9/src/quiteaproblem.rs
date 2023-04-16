use std::io::{self, BufRead};

pub fn main() {
    let a = 43;
    let b: u64 = 7879;
    // let a = 3;
    // let b = 97;

    let prob_str: &str = "problem";
    let (problem_str_h, _) = preprocessing(prob_str, a, b);
    let problem_hash_value = problem_str_h[6];

    let mut print_vec: Vec<&str> = vec![];

    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let mut print_str = "no";
        let words: Vec<&str> = line.split_whitespace().collect();
        for word in words {
            println!("{word}");
            if word.len() >= prob_str.len() {
                let (h, p) = preprocessing(word, a, b);
                if has_problem_sub_str(&h, &p, b, problem_hash_value) {
                    print_str = "yes";
                    break;
                }
            }
        }
        print_vec.push(print_str);
    }
    for s in print_vec {
        println!("{}", s);
    }
}

fn preprocessing(word: &str, a: u64, b: u64) -> (Vec<u64>, Vec<u64>) {
    let n = word.len();
    let mut chars = word.chars();
    let mut h: Vec<u64> = Vec::with_capacity(n);
    h.push(chars.next().unwrap().to_ascii_lowercase() as u64);

    let mut p: Vec<u64> = Vec::with_capacity(n);
    p.push(1);
    for _ in 1..n {
        let c = chars.next().unwrap();
        let c_val = c.to_ascii_lowercase() as u64;
        let h_val = h.last().unwrap() * a + c_val % b;
        h.push(h_val);
        let p_val = p.last().unwrap() * a % b;
        p.push(p_val);
    }
    return (h, p);
}

fn has_problem_sub_str(h: &Vec<u64>, p: &Vec<u64>, b: u64, problem_hash_value: u64) -> bool {
    if h[6] == problem_hash_value {
        return true;
    } else {
        for i in 1..h.len() - 6 {
            let i_a = i - 1;
            let i_b = i + 6;
            let curr_hash_value = h[i_b] - h[i_a] * p[i_b - i + 1] % b;
            if curr_hash_value == problem_hash_value {
                return true;
            }
        }
    }
    return false;
}

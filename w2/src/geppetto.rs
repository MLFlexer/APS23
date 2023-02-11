use std::{io::{self, Read}, collections::HashSet};

pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    
    let n : i16 = input.next().unwrap().parse().unwrap();
    let m : i16 = input.next().unwrap().parse().unwrap();
    let mut set : HashSet<(i16, i16)> = HashSet::new();
    for _ in 0..m {
        let i : i16 = input.next().unwrap().parse::<i16>().unwrap() - 1;
        let j : i16 = input.next().unwrap().parse::<i16>().unwrap() - 1;
        if i < j {
            set.insert((i, j));
        } else {
            set.insert((j, i));
        }
    }
    let mut subset : Vec<i16> = vec![];
    println!("{}", search(&mut subset, 0, &set, n));
}

fn search(set : &mut Vec<i16>, k : i16, forbidden : &HashSet<(i16, i16)>, n : i16) -> i64 {
    if k == n {
        return 1;
    } else {
        let mut count = 0;
        count += search(set, k + 1, forbidden, n);

        for i in set.iter() {
            if forbidden.contains(&(*i, k)) {
                return count;
            }
        }
        set.push(k);
        count += search(set, k + 1, forbidden, n);
        set.pop();
        return count;
    }
}

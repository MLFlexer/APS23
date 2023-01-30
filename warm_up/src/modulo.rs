use std::{io::{self, BufRead}, collections::HashSet};

pub fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    
    let mut set : HashSet<u8> = HashSet::new();

    for _ in 0..10 {
        let n : u16 = (iterator.next().unwrap().unwrap())
            .parse::<u16>().unwrap(); 

        let x : u8 = (n % 42) as u8;
        set.insert(x);
    }

    println!("{}", set.len());
}

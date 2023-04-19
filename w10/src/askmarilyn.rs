use std::{
    io::{self, BufRead},
    process::exit,
};

pub fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines().map(|l| l.unwrap());
    let options = ["A", "B", "C"];
    for _ in 0..1000 {
        let rnd_pointer = Box::into_raw(Box::new(42)) as usize;
        let choise = options[rnd_pointer % 3];
        println!("{choise}");
        let line = iter.next().unwrap();
        let respondse: Vec<&str> = line.split_whitespace().collect();
        let door = respondse[0];
        let correctness = respondse[1];
        if correctness == "1" {
            println!("{door}");
        } else {
            for c in options {
                if c != choise && c != door {
                    println!("{c}")
                }
            }
        }
        iter.next().unwrap();
    }
    exit(0x100);
}

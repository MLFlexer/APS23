use std::io::{self, Read};

pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    
    let n : i32 = input.next().unwrap().parse().unwrap();
    let t : i8 = input.next().unwrap().parse().unwrap();
    let mut lst : Vec<(i32, i8)> = vec![];
    for _ in 0..n {
        let money : i32 = input.next().unwrap().parse().unwrap();
        let wait : i8 = input.next().unwrap().parse().unwrap(); 
        lst.push((money, wait))
    }

    lst.sort_by(|(c_a, _), (c_b, _)| c_a.partial_cmp(c_b).unwrap());

    let mut timeslots : Vec<i32> = vec![0; t as usize];
    
    for _ in 0..=n {
        match lst.pop() {
            Some((money, wait)) => {
                let mut min = t - 1;
                if wait < t {
                    min = wait;
                }
                for i in 0..=min {
                    let index = timeslots[(min - i) as usize];
                    if index == 0 {
                        timeslots[(min - i) as usize] = money; 
                        // println!("{:?}", timeslots);
                        break;
                    }
                }
            },
            None => break,
        }
    }
    println!("{}", timeslots.iter().sum::<i32>())
}

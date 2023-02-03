use std::{io::{self, Read}, iter::OnceWith};

pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    
    let n : usize = input.next().unwrap().parse().unwrap();

    let mut busnumbers : Vec<u16> = vec![];

    for _ in 0..n {
        let v = input.next().unwrap().parse::<u16>().unwrap(); 
        busnumbers.push(v);
    }

    busnumbers.sort();

    print_busses(busnumbers, 0, 0);
    
}

fn print_busses(busses : Vec<u16>, i : usize, x : usize) -> () {
    let cur = busses[i];

    if (i + 1 != busses.len() && busses[i + 1] == cur + 1) {
        if x == 0 {
            print!("{}", cur);
        }
        if i + 1 == busses.len() {return;}
        print_busses(busses, i+1, x+1)
    } else {
        if x > 1 {
            print!("-{} ", cur);
        } else {
            print!(" {} ", cur);
        }
        if i + 1 == busses.len() {return;}
        print_busses(busses, i+1, 0);
    }
}
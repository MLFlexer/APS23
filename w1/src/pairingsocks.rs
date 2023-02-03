use std::{io::{self, Read}, iter::OnceWith, collections::LinkedList};

pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    
    let n : u32 = input.next().unwrap().parse().unwrap();

    let mut org : LinkedList<u32> = LinkedList::new();
    let mut aux : LinkedList<u32> = LinkedList::new();

    for _ in 0..2*n {
        let v = input.next().unwrap().parse::<u32>().unwrap(); 
        org.push_front(v);
    }

    let mut moves = 0;
    

    for _ in 0..((2*n) + 1) {
        let x = org.pop_front();
        let y = aux.pop_front();

        match (x, y) {
            (None, None) => println!("{}", moves),
            (None, Some(_)) => println!("impossible"),
            (Some(a), Some(b)) => {
                if a == b {
                    moves += 1;
                } else {
                    aux.push_front(b);
                    aux.push_front(a);
                    moves += 1;
                }
            },
            (Some(a), None) => 
                {
                    aux.push_front(a);
                    moves += 1;
                },
            }
    }
}
    
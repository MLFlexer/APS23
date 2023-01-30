use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::io;



fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    io::stdin().read_line(&mut line).unwrap();
    let first_line_vec : Vec<&str> = line.split_whitespace().collect();
    println!("{:?}", first_line_vec);
    let mut min_heap : BinaryHeap<Reverse<u16>> = BinaryHeap::from_iter(first_line_vec[1..].iter().map(|x| Reverse(x.parse::<u16>().unwrap())));
    print!("{:?}", min_heap.iter());
    let prev : u16 = min_heap.pop().unwrap().0;
    print!("{}", prev);
    make_output(&mut min_heap, prev);
}

fn make_output(min_heap : &mut BinaryHeap<Reverse<u16>>, prev : u16) -> () {
    println!("i : {}", prev);
    if let opt = min_heap.pop().unwrap() {
        let x = opt.0;
        if x == prev + 1 {
            if x + 1 != min_heap.pop().unwrap().0 {
                print!("-{}", x);
                make_output(min_heap, x);
            } else {
                make_output(min_heap, x);
            }
        } else {
            print!("{}", prev);
            make_output(min_heap, x);
        }
    }
}
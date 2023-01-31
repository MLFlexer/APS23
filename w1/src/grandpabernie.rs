use std::{io::{self, Read}, collections::HashMap};

pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    
    let n : u32 = input.next().unwrap().parse().unwrap();
    let mut map : HashMap<String, Vec<u32>>= HashMap::new();
    for _ in 0..n {
        let k = input.next().unwrap().to_string();
        let v = input.next().unwrap().parse::<u32>().unwrap(); 
        if let Some(vec) = map.get_mut(&k)  {
            vec.push(v);
        } else {
            map.insert(k, Vec::from([v]));
        }
    }

    map.values_mut().map(|x| x.sort()).last();

    let m : u32 = input.next().unwrap().parse().unwrap();

    for _ in 0..m {
        let k = input.next().unwrap().to_string();
        let v = input.next().unwrap().parse::<usize>().unwrap(); 
        
        let vec = map.get(&k).unwrap();
        println!("{}", vec[v - 1]);
    }

}

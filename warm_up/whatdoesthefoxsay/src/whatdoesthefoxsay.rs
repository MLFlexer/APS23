use std::{io::{self, BufRead, Lines}, collections::HashSet};

pub fn main() {
    let stdin = io::stdin();
    let mut iterator : Lines<io::StdinLock>= stdin.lock().lines();
    
    let n : u32 = (iterator.next().unwrap().unwrap()).parse::<u32>().unwrap();

    for _i in 0..n {
        let sound_str = &mut iterator.next().unwrap().unwrap();
        
        let sounds = sound_str.split(" ").into_iter();

        let mut set : HashSet<String>= HashSet::new();
        loop {
            let line = &mut iterator.next().unwrap().unwrap();

            if "what does the fox say?" == &*line {
                sounds.clone().for_each(|e| {
                    if !set.contains(&e.to_string()) {print!("{} ", e);}
                });
                break;
            } 
            let kv_pairs : Vec<&str> = (line).split(" ").collect();
            set.insert(kv_pairs[2].to_string());
            
        }
    }
}

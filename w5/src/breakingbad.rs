use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

#[derive(Copy, Clone, Debug, PartialEq)]
enum Colors {
    Red,
    Blue,
    Undefined,
}

pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n: i32 = input.next().unwrap().parse().unwrap();

    let mut set: HashSet<&str> = HashSet::new();

    for _ in 0..n {
        let item = input.next().unwrap();
        set.insert(item);
    }

    let m: i32 = input.next().unwrap().parse().unwrap();
    let mut map: HashMap<&str, (Colors, Vec<&str>)> = HashMap::new();

    for _ in 0..m {
        let i = input.next().unwrap();
        let j = input.next().unwrap();

        match map.get_mut(i) {
            None => {
                map.insert(i, (Colors::Undefined, vec![j]));
            }
            Some((_, v)) => {
                v.push(j);
            }
        }

        match map.get_mut(j) {
            None => {
                map.insert(j, (Colors::Undefined, vec![i]));
            }
            Some((_, v)) => {
                v.push(i);
            }
        }
    }

    for e in set.iter() {
        if color_graph(*e, &mut map) {
            println!("impossible");
            return;
        }
    }

    let mut red: Vec<&str> = vec![];
    let mut blue: Vec<&str> = vec![];

    for name in set {
        match map.get(name) {
            None => blue.push(name),
            Some((color, _)) => {
                //println!("color: {:?}", color);
                match color {
                    Colors::Red => {
                        red.push(name);
                    }
                    Colors::Blue => {
                        blue.push(name);
                    }
                    Colors::Undefined => blue.push(name),
                }
            }
        }
    }

    for s in blue {
        print!("{} ", s);
    }
    println!("");
    for s in red {
        print!("{} ", s);
    }
}

fn color_graph<'a>(start_node: &str, map: &mut HashMap<&'a str, (Colors, Vec<&'a str>)>) -> bool {
    //println!("start_node: {}", start_node);
    //println!("map: {:?}", map);

    let map_node = map.get_mut(start_node);
    if map_node == None {
        //println!("NONE");
        return false;
    }
    let map_node = map_node.unwrap();

    if map_node.0 != Colors::Undefined {
        return false;
    }

    map_node.0 = Colors::Blue;
    //println!("coloring blue: {:?}", map_node);

    let mut queue: Vec<&str> = vec![start_node];

    while !queue.is_empty() {
        let curr = queue.pop().unwrap();
        //let curr = map.get(curr).unwrap();
        let curr_color = map.get(curr).unwrap().0;

        for node in map.get(curr).unwrap().1.clone() {
            let node_color = map.get(node).unwrap().0;

            if node_color == curr_color {
                return true;
            } else if node_color == Colors::Undefined {
                map.get_mut(node).unwrap().0 = if curr_color == Colors::Blue {
                    Colors::Red
                } else {
                    Colors::Blue
                };
                queue.push(node);
            }
        }
    }
    return false;
}

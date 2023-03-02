use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split("\n");

    let n: i32 = input.next().unwrap().parse().unwrap();
    println!("n: {}", n);

    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    for _ in 0..n {
        let mut stations = input.next().unwrap().split(" ");
        let node = stations.next().unwrap();
        let edges: Vec<&str> = stations.collect();
        println!("edges: {:?}", &edges);
        let e_iter = edges.clone();
        for e in e_iter {
            match map.get(e) {
                None => {
                    let v = vec![node];
                    map.insert(e, v);
                }
                Some(v) => {
                    v.push(node);
                }
            }
        }
        map.insert(node, edges);
        println!("node: {}", node);
    }
    let mut last_iter = input.next().unwrap().split(" ");

    let start = last_iter.next().unwrap();
    println!("{}", start);
    let end = last_iter.next().unwrap();
    println!("{}", end);
    let set: HashSet<&str> = HashSet::new();
    let mut directions: Vec<&str> = vec![];
    directions.push(start);
    match search(start, end, &map, directions, &set) {
        None => {
            println!("no route found");
        }
        Some(v) => {
            println!("result = {:?}", v);
        }
    }
}

fn search<'a>(
    node: &str,
    end_node: &'a str,
    map: &HashMap<&str, Vec<&'a str>>,
    directions: Vec<&'a str>,
    visited: &HashSet<&str>,
) -> Option<Vec<&'a str>> {
    println!("{}", node);
    let edges: &Vec<&str> = map.get(node).unwrap();
    let mut visited = visited.clone();
    visited.insert(node);
    for edge in edges {
        if visited.contains(edge) {
            continue;
        }
        let mut directions = directions.clone();
        directions.push(edge);
        if *edge == end_node {
            return Some(directions);
        }
        let res = search(edge, end_node, map, directions, &visited);
        match res {
            None => {}
            Some(_) => return res,
        }
    }
    return None;
}

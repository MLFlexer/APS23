use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{self, Read},
    vec,
};
#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Edge {
    capacity: i32,
    flow: i32,
}

impl Edge {
    fn get_remainging_flow(&self) -> i32 {
        println!("remaining: {}", (self.capacity - self.flow));
        println!("capacity: {}", self.capacity);
        return self.capacity - self.flow;
    }
}

pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();
    let m: usize = input.next().unwrap().parse().unwrap();
    let s: usize = input.next().unwrap().parse().unwrap();
    let t: usize = input.next().unwrap().parse().unwrap();

    let mut graph: Vec<HashMap<usize, Edge>> = vec![HashMap::new(); n];
    let mut max_cap = 0;
    for _ in 0..m {
        let u: usize = input.next().unwrap().parse().unwrap();
        let v: usize = input.next().unwrap().parse().unwrap();
        let c: i32 = input.next().unwrap().parse().unwrap();
        if c > max_cap {
            max_cap = c;
        }
        graph[u].insert(
            v,
            Edge {
                capacity: c,
                flow: 0,
            },
        );
        graph[v].insert(
            u,
            Edge {
                capacity: 0,
                flow: 0,
            },
        );
    }
    let (max_flow, min_cut_nodes) = solve_flow(&mut graph, s, t, max_cap);
    println!("max_flow: {:?}", max_flow);
    println!("min_cut_nodes: {:?}", min_cut_nodes);
    println!("graph: {:?}", graph);

    for from_node in 0..graph.len() {
        for (to_node, edge) in &graph[from_node] {
            let cap = edge.flow;

            if edge.flow <= edge.capacity && edge.capacity != 0 {
                println!("{from_node} {to_node} {cap}");
            }
        }
    }
}

fn bfs(
    graph: &Vec<HashMap<usize, Edge>>,
    source: usize,
    sink: usize,
    cap: i32,
) -> Option<Vec<(usize, usize)>> {
    //TODO: bør returnere den mindste capacity, som kan flowes
    //igennem, så man undgår at skulle løbe det igennem på linje
    //88/98
    let mut parents: HashMap<usize, usize> = HashMap::new();
    let mut queue: VecDeque<usize> = VecDeque::from([source]);
    while !queue.is_empty() {
        let from_node = queue.pop_front().unwrap();
        for (to_node, edge) in graph[from_node].clone() {
            if edge.get_remainging_flow() >= cap && !parents.contains_key(&to_node) {
                parents.insert(to_node, from_node);
                queue.push_back(to_node);
                if to_node == sink {
                    let mut path: Vec<(usize, usize)> = vec![];
                    let mut curr_node = sink;
                    while curr_node != source {
                        let tmp = *parents.get(&curr_node).unwrap();
                        path.push((tmp, curr_node));
                        curr_node = tmp;
                    }
                    return Some(path);
                }
            }
        }
    }
    return None;
}

fn solve_flow(
    graph: &mut Vec<HashMap<usize, Edge>>,
    source: usize,
    sink: usize,
    max_cap: i32,
) -> (i32, Vec<(usize, usize)>) {
    let mut curr_cap = max_cap;
    let mut bottle_neck = i32::MAX;
    let mut max_flow = 0;
    let mut path: Vec<(usize, usize)> = vec![];
    loop {
        match bfs(&graph, source, sink, curr_cap) {
            Some(found_path) => {
                {
                    for (from_node, to_node) in &found_path {
                        bottle_neck = bottle_neck.min(
                            graph[*from_node]
                                .get(&to_node)
                                .unwrap()
                                .get_remainging_flow(),
                        );
                    }
                }

                max_flow += bottle_neck;

                for (from_node, to_node) in &found_path {
                    //TODO: husk at sætte residual graph, hvis det er fejl!
                    graph[*from_node].get_mut(&to_node).unwrap().flow += bottle_neck;
                    graph[*to_node].get_mut(&from_node).unwrap().flow += bottle_neck;
                }
                path = found_path;
            }
            None => {
                if curr_cap > 0 {
                    curr_cap /= 2;
                }
                if curr_cap < 1 {
                    return (max_flow, path);
                }
            }
        }
    }
}

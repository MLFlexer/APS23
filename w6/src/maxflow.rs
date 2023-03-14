use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{self, Read},
};

#[derive(Debug, PartialEq, PartialOrd, Clone, Ord, Eq)]
struct Edge {
    capacity: i64,
    flow: i64,
}

impl Edge {
    fn get_remainging_flow(&self) -> i64 {
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
        let c: i64 = input.next().unwrap().parse().unwrap();
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
    }
    let (max_flow, _) = solve_flow(&mut graph, s, t, max_cap);
    let mut max_flow_edges: HashSet<(usize, usize, i64)> = HashSet::new();
    for from_node in 0..graph.len() {
        for (to_node, edge) in &graph[from_node] {
            if edge.flow > 0 && edge.capacity > 0 {
                max_flow_edges.insert((from_node, *to_node, edge.flow));
            }
        }
    }

    println!("{} {} {}", n, max_flow, max_flow_edges.len());

    for line in max_flow_edges {
        println!("{} {} {}", line.0, line.1, line.2);
    }
}

fn bfs(
    graph: &Vec<HashMap<usize, Edge>>,
    source: usize,
    sink: usize,
    cap: i64,
) -> Option<(i64, Vec<(usize, usize)>)> {
    let mut parents: HashMap<usize, usize> = HashMap::new();
    let mut queue: VecDeque<usize> = VecDeque::from([source]);
    while !queue.is_empty() {
        let from_node = queue.pop_front().unwrap();
        for (to_node, edge) in &graph[from_node] {
            if edge.get_remainging_flow() >= cap && !parents.contains_key(&to_node) {
                parents.insert(*to_node, from_node);
                queue.push_back(*to_node);
                if *to_node == sink {
                    let mut path: Vec<(usize, usize)> = vec![];
                    let mut curr_node = sink;
                    let mut bottle_neck = edge.get_remainging_flow();
                    while curr_node != source {
                        let tmp = *parents.get(&curr_node).unwrap();
                        bottle_neck = bottle_neck
                            .min(graph[tmp].get(&curr_node).unwrap().get_remainging_flow());
                        path.push((tmp, curr_node));
                        curr_node = tmp;
                    }
                    return Some((bottle_neck, path));
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
    max_cap: i64,
) -> (i64, Vec<(usize, usize)>) {
    let mut curr_cap = max_cap;
    let mut max_flow = 0;
    let mut path: Vec<(usize, usize)> = vec![];
    loop {
        match bfs(&graph, source, sink, curr_cap) {
            Some((bottle_neck, found_path)) => {
                max_flow += bottle_neck;
                for (from_node, to_node) in &found_path {
                    graph[*from_node].get_mut(&to_node).unwrap().flow += bottle_neck;
                    match graph[*to_node].get_mut(&from_node) {
                        Some(edge) => {
                            edge.flow -= bottle_neck;
                        }
                        None => {
                            graph[*to_node].insert(
                                *from_node,
                                Edge {
                                    capacity: 0,
                                    flow: -bottle_neck,
                                },
                            );
                        }
                    }
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

use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{self, Read},
    vec,
};

pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();
    let m: usize = input.next().unwrap().parse().unwrap();
    let s: usize = input.next().unwrap().parse().unwrap();
    let t: usize = input.next().unwrap().parse().unwrap();

    let mut graph: Vec<HashMap<usize, i64>> = vec![HashMap::new(); n];
    let mut r_graph: Vec<HashMap<usize, i64>> = vec![HashMap::new(); n];
    let mut max_cap = 0;
    for _ in 0..m {
        let u: usize = input.next().unwrap().parse().unwrap();
        let v: usize = input.next().unwrap().parse().unwrap();
        let c: i64 = input.next().unwrap().parse().unwrap();
        if c > max_cap {
            max_cap = c;
        }
        r_graph[u].insert(v, 0);
        r_graph[v].insert(u, 0);
        graph[u].insert(v, c);
        graph[v].insert(u, 0);
    }
    let (max_flow, min_cut_nodes) = solve_flow(&mut graph, &mut r_graph, s, t, max_cap);
    // println!("max_flow: {:?}", max_flow);
    // println!("min_cut_nodes: {:?}", min_cut_nodes);
    // println!("graph: {:?}", graph);
    let mut lines: Vec<String> = vec![];
    let mut max_flow_edges: HashSet<(usize, usize, i64)> = HashSet::new();
    let mut num_edges = 0;
    // for from_node in 0..r_graph.len() {
    //     for (to_node, edge) in &r_graph[from_node] {
    //         let cap = *edge;
    //         let flow = graph[from_node].get(to_node).unwrap();
    //         if cap > 0 {
    //             // if flow <= cap && cap != 0 && flow > 0 {
    //             //     let cap_flow = cap - flow;
    //
    //             max_flow_edges.insert((from_node, *to_node, cap));
    //             num_edges += 1;
    //
    //             //                lines.push(format!("{from_node} {to_node} {flow}"));
    //
    //             //println!("{from_node} {to_node} {cap}");
    //         }
    //     }
    // }
    for to_node in 0..r_graph.len() {
        for (from_node, flow) in &r_graph[to_node] {
            if *flow > 0 {
                num_edges += 1;
                max_flow_edges.insert((*from_node, to_node, *flow));
            }
        }
    }

    let m_lines = lines.len();
    //println!("{:?}", min_cut_nodes);
    println!("{n} {max_flow} {num_edges}");

    let mut max_flow_edges: Vec<(usize, usize, i64)> = max_flow_edges.into_iter().collect();
    max_flow_edges.sort();

    for line in max_flow_edges {
        println!("{} {} {}", line.0, line.1, line.2);
    }
}

fn bfs(
    graph: &Vec<HashMap<usize, i64>>,
    source: usize,
    sink: usize,
    cap: i64,
) -> Option<(i64, Vec<(usize, usize)>)> {
    //TODO: bør returnere den mindste capacity, som kan flowes
    //igennem, så man undgår at skulle løbe det igennem på linje
    //88/98
    let mut parents: HashMap<usize, usize> = HashMap::new();
    let mut queue: VecDeque<usize> = VecDeque::from([source]);
    while !queue.is_empty() {
        let from_node = queue.pop_front().unwrap();
        for (to_node, flow) in graph[from_node].clone() {
            if flow >= cap && !parents.contains_key(&to_node) {
                parents.insert(to_node, from_node);
                queue.push_back(to_node);
                if to_node == sink {
                    let mut path: Vec<(usize, usize)> = vec![];
                    let mut curr_node = sink;
                    let mut bottle_neck = flow;
                    while curr_node != source {
                        let tmp = *parents.get(&curr_node).unwrap();
                        // println!("bottle_neck: {}", bottle_neck);
                        bottle_neck = bottle_neck.min(*graph[tmp].get(&curr_node).unwrap());
                        path.push((tmp, curr_node));
                        curr_node = tmp;
                    }
                    // println!("bottle_neck: {}", bottle_neck);
                    return Some((bottle_neck, path));
                }
            }
        }
    }
    return None;
}

fn solve_flow(
    graph: &mut Vec<HashMap<usize, i64>>,
    r_graph: &mut Vec<HashMap<usize, i64>>,
    source: usize,
    sink: usize,
    max_cap: i64,
) -> (i64, Vec<(usize, usize)>) {
    let mut curr_cap = max_cap;
    //let mut bottle_neck = i64::MAX;
    let mut max_flow = 0;
    let mut path: Vec<(usize, usize)> = vec![];
    loop {
        match bfs(&graph, source, sink, curr_cap) {
            Some((bottle_neck, found_path)) => {
                // {
                //     println!("bottle_neck bfs: {}", bottle_neck);
                //     for (from_node, to_node) in &found_path {
                //         bottle_neck = bottle_neck.min(
                //             graph[*from_node]
                //                 .get(&to_node)
                //                 .unwrap()
                //                 .get_remainging_flow(),
                //         );
                //     }
                //     println!("bottle_neck: {}", bottle_neck);
                // }

                max_flow += bottle_neck;

                for (from_node, to_node) in &found_path {
                    //TODO: husk at sætte residual graph, hvis det er fejl!
                    *graph[*from_node].get_mut(&to_node).unwrap() -= bottle_neck;
                    *graph[*to_node].get_mut(&from_node).unwrap() += bottle_neck;
                    *r_graph[*from_node].get_mut(&to_node).unwrap() -= bottle_neck;
                    *r_graph[*to_node].get_mut(&from_node).unwrap() += bottle_neck;
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

use std::{
    cell::RefCell,
    io::{self, Read},
    rc::Rc,
};

#[derive(Debug, Clone)]
struct Edge {
    from: usize,
    to: usize,
    flow: i32,
    capacity: i32,
}

pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();
    let m: i32 = input.next().unwrap().parse().unwrap();
    let s: i32 = input.next().unwrap().parse().unwrap();
    let t: i32 = input.next().unwrap().parse().unwrap();

    let mut nodes: Vec<Vec<Edge>> = vec![Vec::new(); n];
    let mut max_cap: i32 = 0;
    for _ in 0..m {
        let u: usize = input.next().unwrap().parse().unwrap();
        let v: usize = input.next().unwrap().parse().unwrap();
        let c: i32 = input.next().unwrap().parse().unwrap();
        match nodes.get_mut(u) {
            None => {}
            Some(edges) => edges.push(Edge {
                from: u,
                to: v,
                flow: 0,
                capacity: c,
            }),
        }
        match nodes.get_mut(v) {
            None => {}
            Some(edges) => edges.push(Edge {
                from: v,
                to: u,
                flow: 0,
                capacity: 0,
            }),
        }
        if max_cap < c {
            max_cap = c;
        }
    }

    let mut threshold: i32 = 2_i32.pow(27); //power of 2 which is closest to 10⁸

    while max_cap < threshold {
        threshold = threshold / 2;
    }
}

fn dfs(
    node: usize,
    nodes: Rc<RefCell<Vec<Vec<Edge>>>>,
    visited: &Vec<bool>,
    flow: i32,
    t: usize,
    threshold: i32,
) -> i32 {
    if node == t {
        return flow;
    }

    let edges: &mut Vec<Edge> = nodes.get_mut().get_mut(node).unwrap();
    //let edges: &mut Vec<&mut Edge> = nodes.get_mut(node).unwrap();

    for edge in edges {
        let cap = edge.capacity - edge.flow;
        if (cap >= threshold && !visited[edge.to]) {
            let max_flow = dfs(edge.to, nodes, visited, flow.min(cap), t, threshold);
            if max_flow > 0 {
                (*edge).flow += max_flow;
                //nodes[edge.to][edge.from].flow += max_flow;
            }
        }
    }

    return 0;
}

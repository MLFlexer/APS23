use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

#[derive(Debug, Clone)]
struct Edge {
    from: usize,
    to: usize,
    flow: i64,
    capacity: i64,
}

impl Edge {
    fn get_residual<'a>(&'a self, graph: &'a mut Graph) -> &mut Edge {
        return graph
            .nodes
            .get_mut(self.to)
            .unwrap()
            .get_mut(&self.from)
            .unwrap();
    }
    fn new(from: usize, to: usize, capacity: i64) -> Edge {
        return Edge {
            from,
            to,
            flow: 0,
            capacity,
        };
    }
    fn augment(&mut self, threshold: i64, graph: &mut Graph) {
        self.flow += threshold;
        let mut residual = self.get_residual(graph);
        residual.flow -= threshold;
    }
    fn get_remaining_capacity(&self) -> i64 {
        return self.capacity - self.flow;
    }
}

#[derive(Debug, Clone)]
struct Graph {
    nodes: Vec<HashMap<usize, Edge>>,
    source: usize,
    sink: usize,
    max_cap: i64,
    max_flow: i64,
}

impl Graph {
    fn new(source: usize, sink: usize, size: usize) -> Graph {
        return Graph {
            nodes: vec![HashMap::new(); size],
            source,
            sink,
            max_cap: 0,
            max_flow: 0,
        };
    }
    fn add_edge(&mut self, edge: Edge) -> () {
        self.max_cap = edge.capacity.max(self.max_cap);
        self.nodes.get_mut(edge.from).unwrap().insert(edge.to, edge);
    }
    fn solve(&mut self) {
        let mut threshold: i64 = 2_i64.pow(27); //power of 2 which is closest to 10⁸
        while self.max_cap < threshold {
            threshold = threshold / 2;
        }
        // println!("threshold: {}", threshold);
        let mut visited: Vec<i64> = vec![0; self.nodes.len()];
        let mut visit_id = 1;
        while threshold > 0 {
            let mut f = 0;
            loop {
                f = self.dfs(self.source, i64::MAX, &mut visited, visit_id, threshold);
                // println!("f: {}", f);
                // println!("source: {:?}", self.nodes.get(self.source));
                // println!("sink: {:?}", self.nodes.get(self.sink));
                self.max_flow += f;
                visit_id += 1;
                if f == 0 {
                    break;
                }
            }
            threshold /= 2;
        }
    }
    fn dfs(
        &mut self,
        node: usize,
        flow: i64,
        visited: &mut Vec<i64>,
        visit_id: i64,
        threshold: i64,
    ) -> i64 {
        if node == self.sink {
            return flow;
        }
        visited.insert(node, visit_id);
        let edges = self.nodes.get(node).unwrap().clone();

        for (to_node, edge) in edges {
            let cap = edge.get_remaining_capacity();
            if cap >= threshold && !(visited[edge.to] == visit_id) {
                let ret_flow = self.dfs(edge.to, flow.min(cap), visited, visit_id, threshold);

                if ret_flow > 0 {
                    // edge.flow += threshold;
                    // let mut residual = self
                    //     .nodes
                    //     .get_mut(edge.to)
                    //     .unwrap()
                    //     .get_mut(edge.from)
                    //     .unwrap();
                    //residual.flow -= threshold;
                    // self.flow += threshold;
                    // let mut residual = self.get_residual(graph);
                    // residual.flow -= threshold;
                    {
                        let edge = self.nodes.get_mut(node).unwrap().get_mut(&to_node).unwrap();
                        //edge.augment(ret_flow, self);
                        edge.flow += threshold;
                    }
                    let mut residual = self
                        .nodes
                        .get_mut(edge.to)
                        .unwrap()
                        .get_mut(&edge.from)
                        .unwrap();
                    residual.flow -= threshold;
                    return ret_flow;
                }
            }
        }
        return 0;
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

    let mut graph: Graph = Graph::new(s, t, n);
    for _ in 0..m {
        let u: usize = input.next().unwrap().parse().unwrap();
        let v: usize = input.next().unwrap().parse().unwrap();
        let c: i64 = input.next().unwrap().parse().unwrap();
        graph.add_edge(Edge::new(u, v, c));
        graph.add_edge(Edge::new(v, u, 0));
    }
    let mut threshold: i64 = 2_i64.pow(27); //power of 2 which is closest to 10⁸
    let max_cap: i64 = graph.max_cap;

    while max_cap <= threshold {
        threshold = threshold / 2;
    }
    graph.solve();
    // println!("{}", graph.max_flow);
    // println!("source: {:?}", graph.nodes.get(graph.source));
    // println!("sink: {:?}", graph.nodes.get(graph.sink));
    let mut max_flow = 0;
    for (_, edge) in graph.nodes.get(graph.source).unwrap() {
        if edge.capacity != 0 {
            max_flow += edge.flow;
        }
    }

    let mut flow_graph: HashSet<(usize, usize, i64)> = HashSet::new();
    let mut min_cut_nodes: Vec<usize> = vec![];
    for edges in graph.nodes {
        for (_, edge) in edges {
            if edge.capacity != 0 && edge.flow > 0 {
                flow_graph.insert((edge.from, edge.to, edge.flow));
            }
            if edge.capacity != 0 && edge.flow == edge.capacity {
                println!("{:?}", edge);
                min_cut_nodes.push(edge.from);
            }
        }
    }

    let num_lines = min_cut_nodes.len();

    println!("{num_lines}");
    for i in min_cut_nodes {
        println!("{i}");
    }

    // let num_lines = flow_graph.len();
    //
    // println!("{n} {max_flow} {num_lines}");
    //
    // for (u, v, c) in flow_graph {
    //     println!("{u} {v} {c}");
    // }
}

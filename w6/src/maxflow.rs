use std::{
    io::{self, Read}, slice::SliceIndex, collections::{HashMap, HashSet}
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Edge {
    from: usize,
    to: usize,
    capacity: i32,
    flow: i32,
}

pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();
    let m: i32 = input.next().unwrap().parse().unwrap();
    let s: usize = input.next().unwrap().parse().unwrap();
    let t: usize = input.next().unwrap().parse().unwrap();

    let mut graph: Vec<HashMap<usize, Edge>> = vec![HashMap::new(); n];
    //vector with index = node id containtig hashmap with id of the node the edge is connecting to
    let mut max_cap: i32 = 0;
    for _ in 0..m {
        let u: usize = input.next().unwrap().parse().unwrap();
        let v: usize = input.next().unwrap().parse().unwrap();
        let c: i32 = input.next().unwrap().parse().unwrap();
        let mut map_u = graph.get_mut(u).unwrap();
        map_u.insert(v, Edge {
            from: u,
            to: v,
            flow: c,
            capacity: c,
        });
        let mut map_v = graph.get_mut(v).unwrap();
        map_v.insert(u, Edge {
            from: v,
            to: u,
            flow: 0,
            capacity: 0,
        });
        
        if max_cap < c {
            max_cap = c;
        }
    }

    let mut threshold: i32 = 2_i32.pow(27); //power of 2 which is closest to 10⁸
    while max_cap < threshold {
        threshold = threshold / 2;
    }

    println!("{:?}", flow(&graph, s, t, threshold));

    /*let mut done = false;
    while !done {
        let mut visited: Vec<bool> = vec![false; n];
        if dfs(s, &mut nodes, &mut visited, t, threshold) {
            continue;
        }
        threshold = threshold / 2;
        if threshold == 0 {threshold = 1;}
        let mut visited: Vec<bool> = vec![false; n];
        if threshold == 1 && !dfs(s, &mut nodes, &mut visited, t, threshold) {
            done = true;
        }
    }
    println!("{:?}", nodes);
    let mut f = 0;
    for edge in nodes.get(s).unwrap() {
        f += edge.flow
    }

    let mut m_lines = 0;
    let mut m_vec: Vec<(usize, Edge)> = vec![];
    let mut i = 0;
    for edges in nodes {
        for edge in edges {
            if edge.flow != 0 {
                m_lines += 1;
                m_vec.push((i, edge));
            }
        }
        i += 1;
    }

    m_vec.sort();

    println!("{n} {f} {m_lines}");
    for (from, edge) in m_vec {
        let to = edge.to;
        let cap = edge.capacity;
        println!("{from} {to} {cap}");
    }*/


}

/* fn flow2(graph: &mut Vec<HashMap<usize, Edge>>, s: usize, t: usize, mut threshold: i32) -> i32 {
    let mut curr_flow = 0;
    loop {
        println!("currf: {}", curr_flow);
        //let mut is_path = false;
        //let mut visited: Vec<(usize, usize)> = HashSet::from([(s, s)]);
        let (is_path, visited) = bfs(&graph, s, t, threshold);
        if !is_path {
            if threshold > 0 {
                threshold = threshold / 2;
                continue;
            } else {
                return curr_flow;
            }
        }

        //let p = visited;

        let mut min_flow = i32::MAX;
        for (u, v) in visited.clone() {
            let cap = graph.get(u).unwrap().get(&v).unwrap().flow;
            min_flow = min_flow.min(cap);
        }

        curr_flow += min_flow;

        for (u, v) in visited {
            let e1 = graph.get_mut(u).unwrap().get_mut(&v).unwrap();
            e1.flow -= min_flow;
            let e2 = graph.get_mut(v).unwrap().get_mut(&u).unwrap();
            e2.flow -= min_flow;
        }

        
    }
} */

fn flow(org_graph: &Vec<HashMap<usize, Edge>>, s: usize, t: usize, max_capacity: i32) -> (i32, Vec<HashMap<usize, i32>>, Vec<(usize, usize)>){
    let mut graph: Vec<HashMap<usize, Edge>> = org_graph.clone();
    //let mut max_capacity = 0;

    let mut current_flow = 0;
    let mut min_capacity = max_capacity;

    loop {
        println!("mincap: {}", min_capacity);

        //let mut ret_graph: Vec<HashMap<usize, i32>> = vec![];
        //return (1, ret_graph, vec![(1,1)]);
        println!("mincap: {}", min_capacity);
        println!("reee");
        let (is_path, p_or_seen) = bfs(&graph, s, t, min_capacity);
        if !is_path {
            if min_capacity > 0 {
                println!("mincap: {}", min_capacity);
                min_capacity = min_capacity / 2;
                continue;
            } else {
                let mut ret_graph: Vec<HashMap<usize, i32>> = vec![];
                for i in 0..org_graph.len() {
                    let a = i;
                    let d = org_graph[i].clone();
                    for (b, c) in d {
                        let cap = &graph[a].get(&b).unwrap().flow;
                        if *cap < c.flow {
                            ret_graph[a].insert(b, c.flow-cap);
                        }
                    }
                }
                return (current_flow, ret_graph, p_or_seen);
            }
        }
        let p = p_or_seen;

        let mut saturation = i32::MAX;
        for (u, v) in &p {
            saturation = saturation.min(graph[*u].get(&v).unwrap().flow);
        }

        current_flow += saturation;
        for (u, v) in &p {
            let e1 = graph.get_mut(*u).unwrap().get_mut(&v).unwrap();
            e1.flow -= saturation;
            let e2 = graph.get_mut(*v).unwrap().get_mut(&u).unwrap();
            e2.flow -= saturation;
        }
    }
}

fn bfs(graph: &Vec<HashMap<usize, Edge>>, s: usize, t: usize, threshold: i32) -> (bool, Vec<(usize, usize)>) {
    let mut parent = HashMap::from([(s,s)]);
    let mut layer = vec![s];
    while layer.len() != 0 {
        let mut next_layer: Vec<usize> = vec![];
        for u in layer {
            //println!("uuuu");
            //println!("{:?}", &graph[u]);
            for (v, edge) in &graph[u] {
                //println!("{:?}", &graph[u]);
                if edge.capacity - edge.flow >= threshold && !parent.contains_key(&v) {
                    parent.insert(*v, u);
                    next_layer.push(*v);
                    if *v == t {
                        let mut p: Vec<(usize, usize)> = vec![];
                        let mut curr_node = t;
                        while s != curr_node {
                            let next_node = *parent.get(&curr_node).unwrap();
                            p.push((next_node, curr_node));
                            curr_node = next_node;
                        }
                        return (true, p);
                    }
                }
            }
        }
        layer = next_layer;
    }
    return (false, vec![(s, s)]);
}

/* fn dfs(s: usize, graph: &mut Vec<HashMap<usize, Edge>>, visited: &mut HashSet<usize>, threshold: i32, t: usize) -> (bool, HashSet<(usize, usize)>) {
    println!("s: {}", s);
    let mut traversed: HashSet<(usize, usize)> = HashSet::new();
    visited.insert(s);
    for (v, e) in graph.get(s).unwrap().clone() {
        if !visited.contains(&v) && e.capacity - e.flow >= threshold {
            if v == t {
                traversed.insert((s,v));
                return (true, traversed);
            }
            let (found_path, mut set) = dfs(v, graph, visited, threshold, t);
            if found_path {
                set.insert((s, v));
                return (true, set);
            }
        }
    }
    traversed.insert((s, s));
    return (false, traversed)
} */

/* fn dufars(
    node: usize,
    graph: &mut Vec<HashMap<usize, Edge>>,
    t: usize,
    threshold: i32,
) -> (bool, HashSet<(usize, usize)>) {
    println!("node: {}", node);
    println!("threshold: {}", threshold);
    if *visited.get(node).unwrap() == true {
        println!("visited = true");
        return false;
    }
    if node == t {
        println!("node = t");
        return true;
    }

    visited.insert(node, true);
    let mut i = 0;

    for edge in nodes.get(node).unwrap().clone() {
        if edge.capacity >= threshold && !*visited.get(edge.to).unwrap() {
            let res = dfs(edge.to, nodes, visited, t, threshold);
            if res {
                println!("res: {:?}", nodes);
                let edge = nodes.get_mut(node).unwrap().get_mut(i).unwrap();
                edge.capacity -= threshold;
                edge.flow += threshold;
                println!("res: {:?}", nodes);
                return true;
            } else {
                return false;
            }
        }
        i += 1;
    }
    return false;
}
 */
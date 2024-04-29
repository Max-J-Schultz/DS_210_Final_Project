use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;

fn main() {
    println!("Hello, world!");
    let data = read_data("facebook_combined.txt");
<<<<<<< HEAD
    //let deg_cent = degree_centrality(&data, 24);
    //println!("Degree Centrality: {:?}", deg_cent);
    let deg = modified_bfs(&data, 4);
    println!("{:?}", deg);
    //println!("{:?}", data);
=======
    //println!("{:?}", data);
    let dist = modified_bfs(data, 3);
    println!("{:?}", dist);
>>>>>>> 409d9fa0abb1ed1ac9fd83c2aadd510603a3447c
}

fn read_data(path: &str) -> HashMap<usize, Vec<usize>> {
    let file = File::open(path).expect("Cannot Open File");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    if let Some(Ok(first_line)) = lines.next() {
        for line in lines {
            if let Ok(line_str) = line {
                let mut iter = line_str.trim().split_whitespace();
                if let (Some(u_str), Some(v_str)) = (iter.next(), iter.next()) {
                    let u = u_str.parse::<usize>().expect("Failed to parse integer");
                    let v = v_str.parse::<usize>().expect("Failed to parse integer");
                    graph.entry(u).or_insert_with(Vec::new).push(v);
                    graph.entry(v).or_insert_with(Vec::new).push(u);
                }
            }
        }
    }
    return graph;
}

<<<<<<< HEAD
fn degree_centrality(graph: &HashMap<usize, usize> , start: usize) -> f64 {
    let num_nodes = graph.len();
    if !graph.contains_key(&start) {
    panic!("Starting node not found in the graph");
    }
    let neighbors = match graph.get(&start) {
      Some(neighbor_list) => neighbor_list,
      None => return 0.0, // event that there are no neighbors, the centrality is 0
=======
fn degree_centrality(graph: &HashMap<usize,usize> , starting_node: usize) -> f64 {
    let num_nodes = graph.len();
    if !graph.contains_key(&starting_node) {
    panic!("Starting node not found in the graph");
    }
    let neighbors = match graph.get(&starting_node) {
      Some(neighbor_list) => neighbor_list,
      None => return 0.0, 
>>>>>>> 409d9fa0abb1ed1ac9fd83c2aadd510603a3447c
    };

    let degree = (*neighbors as f64).ln() as f64;
    println!("Number of nodes: {}", num_nodes);
    println!("Degree Centrality: {}", degree);
    return degree; 
}

<<<<<<< HEAD
fn modified_bfs(graph: &HashMap<usize, Vec<usize>>, starting_node: usize) -> Vec<(usize, u32)> {
    let mut distance: Vec<Option<u32>> = vec![None; graph.len()];
    distance[starting_node] = Some(0); // <= we know this distance
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(starting_node);
    let mut result: HashMap<usize, u32> = HashMap::new();
    while let Some(v) = queue.pop_front() {
        if let Some(neighbors) = graph.get(&v) {
            for &u in neighbors.iter() {
                if distance[u].is_none() {
                    distance[u] = Some(distance[v].unwrap() + 1);
                    queue.push_back(u);
                    result.insert(u, distance[u].unwrap());
                }
            }
        }
    }
    let mut sorted_result: Vec<(usize, u32)> = result.into_iter().collect();
    sorted_result.sort_by(|a, b| a.1.cmp(&b.1));
    return sorted_result;
}

=======

fn modified_bfs(graph: &HashMap<usize,usize>, starting_node: usize) {
    let mut distance: Vec<Option<u32>> = vec![None;graph.n];
    distance[starting_node] = Some(0); 
    let mut queue: VecDeque<Vertex> = VecDeque::new();
    queue.push_back(starting_node);
    while let Some(v) = queue.pop_front() { // new unprocessed vertex
        for u in graph.outedges[v].iter() {
            if let None = distance[*u] { // consider all unprocessed neighbors of v
                distance[*u] = Some(distance[v].unwrap() + 1);
                queue.push_back(*u);
            }
        }
    }
    print!("vertex:distance");
    for v in 0..graph.n {
        print!("   {}:{}",v,distance[v].unwrap());
    }
    println!();
    for i in 0..graph.n {
        println!("Distances from node {}", i);
        compute_and_print_distance_bfs(i, &graph);
    }
}


/*fn bfs(graph: HashMap<u32,u32>, node_id: u32) -> HashMap<u32,u32> {
hello world
}
fn recommend_friends(graoh: HashMap<u32,u32>, node_id: u32, max_depth: u32) -> Vec<(u32,u32)> {
>>>>>>> 409d9fa0abb1ed1ac9fd83c2aadd510603a3447c

/* 
recommend_friends()
- takes the adjacency list, node id, and max_depth as inputs/ arguments
- calls bfs function to find closely related users for the specified max_depth
- returns a vector of (nodes, degrees) sorted by smallest degree of separation
*/
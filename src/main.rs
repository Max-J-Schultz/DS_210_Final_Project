use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Hello, world!");
    let data = read_data("facebook_combined.txt");
    //println!("{:?}", data);
    let dist = modified_bfs(data, 3);
    println!("{:?}", dist);
}

fn read_data(path: &str) -> HashMap<usize, usize> {
    let file = File::open(path).expect("Cannot Open File"); 
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut graph: HashMap<usize,usize> = HashMap::new();
    if let Some(Ok(first_line)) = lines.next() {
        for line in lines {
            if let Ok(line_str) = line {
                let mut iter = line_str.trim().split_whitespace();
                if let (Some(u_str), Some(v_str)) = (iter.next(), iter.next()) {
                    let u = u_str.parse::<usize>().expect("Failed to parse integer");
                    let v = v_str.parse::<usize>().expect("Failed to parse integer");
                    graph.insert(u, v);
                }
            }
        }
    }
    return graph;
}

fn degree_centrality(graph: &HashMap<usize,usize> , starting_node: usize) -> f64 {
    let num_nodes = graph.len();
    if !graph.contains_key(&starting_node) {
    panic!("Starting node not found in the graph");
    }
    let neighbors = match graph.get(&starting_node) {
      Some(neighbor_list) => neighbor_list,
      None => return 0.0, 
    };

    let degree = (*neighbors as f64).ln() as f64;
    println!("Number of nodes: {}", num_nodes);
    println!("Degree Centrality: {}", degree);
    return degree; 
}


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

}*/
/* 
Pseudocode:

read_data()
- reads the data from a file path 
- parses txt file line-by-line and extracts the node ids, etc.
- returns and creates the adjacency list — adding the users and their connections

degree_centrality()
- takes the adjacency list and the node id as arguments
- calculates the degree centrality for the user — which is defined as the number of connections
- should return the degree centrality as f32?

bfs()
- takes the graph/ adjacency list and node id as input
- implements BFS from the starting user
- keep track of degree separation for each user
- return hashmap where key is the node id and the value is the degree separation

recommend_friends()
- takes the adjacency list, node id, and max_depth as inputs/ arguments
- calls bfs function to find closely related users for the specified max_depth
- returns a vector of (nodes, degrees) sorted by smallest degree of separation
*/
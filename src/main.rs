use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::VecDeque;
mod connectivity;
use crate::connectivity::degree_centrality;
use crate::connectivity::modified_bfs;
use crate::connectivity::recommend_friends;
use crate::connectivity::influencer;
use crate::connectivity::user_importance;

fn main() {
    let data = read_data("facebook_combined.txt");
    let send_friend_request = recommend_friends(&data, 74, 1.0);
    println!("{:?}", send_friend_request); 
    let determine_importance = user_importance(&data, 74);
    println!("{:?}", determine_importance);
    let influencing_nodes = influencer(&data);
    println!("The top three influential nodes in this network are: {:?}", influencing_nodes);
    //let bfs = modified_bfs(&data, 24);
    //println!{"{:?}", bfs};
}

fn read_data(path: &str) -> HashMap<usize, Vec<usize>> {
    let file = File::open(path).expect("Cannot Open File");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    // my graph will be represented as an adjacency list through my implementation of the HashMap
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    if let Some(Ok(first_line)) = lines.next() {
        for line in lines {
            if let Ok(line_str) = line {
                let mut iter = line_str.trim().split_whitespace();
                if let (Some(u_str), Some(v_str)) = (iter.next(), iter.next()) {
                    // so essentially if there is some (u,v) pair, then it will be parsed and inputted in the correct place
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

#[test]
fn test_recommend_friends() {
    let mut graph = HashMap::new();
    graph.insert(1, vec![2, 3]);
    graph.insert(2, vec![1, 3, 4]);
    graph.insert(3, vec![1, 2, 4]);
    graph.insert(4, vec![2, 3]);
    let recommended_friends = recommend_friends(&graph, 1, 1.0);
    // compiler recommended the use of the .cloned() method for precise type matching in the HashSet
    assert_eq!(recommended_friends, [2, 3].iter().cloned().collect());
}

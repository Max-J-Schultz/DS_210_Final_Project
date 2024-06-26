use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::VecDeque;

pub fn degree_centrality(graph: &HashMap<usize, Vec<usize>> , start: usize) -> f64 {
    let num_nodes = graph.len();
    if !graph.contains_key(&start) {
    panic!("Starting node not found in the graph");
    }
    let neighbors = match graph.get(&start) {
      Some(neighbor_list) => neighbor_list,
      None => return 0.0, // in the event that there are no neighbors, the centrality is 0
    };
    // the degree cetrality here will be defined as the number of neigbors pointing at it
    let degree = (*neighbors).len() as f64; 
    return degree; 
}

pub fn modified_bfs(graph: &HashMap<usize, Vec<usize>>, starting_node: usize) -> Vec<(usize, u32)> {
    let mut distance: HashMap<usize, u32> = HashMap::new();
    distance.insert(starting_node, 0); 
    // the VecDeque will initialize a queue — whatever you put in comes out in the same order 
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(starting_node);
    let mut result: HashMap<usize, u32> = HashMap::new();
    // we will use a while loop so that it runs until there is nothing left to consider 
    while let Some(v) = queue.pop_front() {
        if let Some(neighbors) = graph.get(&v) {
            // here we are looking at the neighbors and visiting the neighbor whose distance is none
            // this essentially means that they are unseen — i.e the crux of BFS 
            for &u in neighbors.iter() {
                if !distance.contains_key(&u) {
                    let dist = *distance.get(&v).unwrap() + 1;
                    distance.insert(u, dist);
                    queue.push_back(u);
                    result.insert(u, dist);
                }
            }
        }
    }
    
    let mut sorted_result: Vec<(usize, u32)> = result.into_iter().collect();
    // this is a little complicated but essentially what i did was use a closure we learned about to sort the second element
    // of the result variable in ascending order - this is so we have the degrees of separation sorted 
    sorted_result.sort_by(|a, b| a.1.cmp(&b.1));
    return sorted_result;
}

pub fn recommend_friends(graph: &HashMap<usize, Vec<usize>>, starting_node: usize, max_difference: f64) -> HashSet<usize> {
    let bfs_result = modified_bfs(graph, starting_node);
    let starting_node_centrality = degree_centrality(graph, starting_node);
    let mut recommendations: Vec<(usize, u32)> = Vec::new();
    for &(node, distance) in &bfs_result {
        let centrality = degree_centrality(graph, node);
        // i am iteratively going through each result of the bfs and degree centrality function and pusing it to the recomendations under 
        // the certain constraints defined here 
        if centrality > starting_node_centrality - max_difference  && centrality < starting_node_centrality + max_difference {
            recommendations.push((node, distance));
        }
    }

    let mut updated_recommendations: Vec<(usize, u32)> = Vec::new();
    let mut recommended_friends: HashSet<usize> = HashSet::new();
    for &mut (node, _) in &mut recommendations {
        let friends = &graph[&node]; // get direct friends
        for &friend in friends {
            let friend_centrality = degree_centrality(graph, friend);
            // these are added constraints to ensure only true recommendations are returned.. it must be limited 
            if friend_centrality > starting_node_centrality - max_difference{
                let friend_distance = bfs_result.iter().find(|&&x| x.0 == friend).unwrap().1;
                if !recommended_friends.contains(&friend){
                    updated_recommendations.push((friend, friend_distance));
                    recommended_friends.insert(friend);
                }
            }
        }
    }
    // here i am limiting the amount of recommendations to 10 — as this is most common for applications 
    // this shouldn't be a problem since it is stored in a HashSet where order doesn't inherently matter 
    let friend_rec: HashSet<usize> = updated_recommendations.into_iter().map(|(friend, _)| friend).take(10).collect();
    println!("These are the users we recommend:");
    return friend_rec;
}

pub fn influencer(graph: &HashMap<usize, Vec<usize>>) -> Vec<usize> {
    let mut top_centralities: Vec<(usize, f64)> = Vec::new();
    for (&node, _) in graph {
        // calculate degree centrality iterativly for current nodes
        let centrality = degree_centrality(graph, node);
        // push the node and its centrality to the vector
        top_centralities.push((node, centrality));
    }
    // again we are using these closures and the sort method to sont them in ascending order
    top_centralities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    // then we are defining the influential users as the top three node with the highest degree centralities 
    let top_nodes: Vec<usize> = top_centralities.iter().map(|&(node, _)| node).take(3).collect();
    top_nodes
}

pub fn user_importance(graph: &HashMap<usize, Vec<usize>>, starting_node: usize) {
    let starting_degree: f64 = degree_centrality(graph, starting_node);
    let influencers: Vec<usize> = influencer(graph);
    if let Some(top_centrality) = influencers.first().map(|&node| degree_centrality(graph, node)){
        let lower = top_centrality * 0.90;
        // obviously no node can have a cetrality higher than the top_centrality as we have previously defined it
        // but this is purely meant as an interval 
        let upper = top_centrality * 1.10;
        // these are the final constraint determining if teh starting_node is essentiall introverted or extroverted 
        if starting_degree >= lower && starting_degree <= upper {
            println!("Additionally, this user is among the most popular in the network — they are probably not shy!");
        } else {
            println!("Additionally, this user isn't very connected — they are probably shy!");
            // for reference, i am only printing this within the function to limit the code in main 
        }
    }
}

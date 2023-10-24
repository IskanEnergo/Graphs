use crate::graph::{Graph, Vertex};
use std::collections::{HashSet, VecDeque};

pub fn bfs(graph: &dyn Graph, start: Vertex) -> HashSet<Vertex> {
    let mut queue = VecDeque::<Vertex>::new();
    let mut visited = HashSet::<Vertex>::new();

    queue.push_back(start);
    visited.insert(start);

    while !queue.is_empty() {
        let v = queue.pop_front().unwrap();

        for neighbor in graph.adjacency_list(v) {
            if visited.contains(&neighbor) { continue; }
            queue.push_back(neighbor);
            visited.insert(neighbor);
        }
    }

    visited
}

pub fn is_graph_strong_connected(graph: &dyn Graph) -> bool {
    for i in 0..graph.count_vertex() {
        if bfs(graph, i).len() != graph.count_vertex() {
            return false
        }
    }
    true
}
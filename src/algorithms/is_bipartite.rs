use std::collections::VecDeque;
use crate::graph::Graph;

pub fn is_bipartite(graph: &dyn Graph) -> (bool, Option<Vec<i32>>) {
    let adj_matrix: Vec<Vec<i32>> = graph.adjacency_matrix();
    let n = adj_matrix.len();
    let mut color = vec![-1; n];
    let mut q = VecDeque::new();
    for i in 0..n {
        if color[i] == -1 {
            color[i] = 0;
            q.push_back(i);
            while !q.is_empty() {
                let u = q.pop_front().unwrap();
                for v in 0..n {
                    if adj_matrix[u][v] == 1 && color[v] == -1 {
                        color[v] = 1 - color[u];
                        q.push_back(v);
                    } else if adj_matrix[u][v] == 1 && color[v] == color[u] {
                        return (false, None);
                    }
                }
            }
        }
    }
    (true, Some(color))
}
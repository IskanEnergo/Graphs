use std::collections::VecDeque;
use crate::graph::{Graph, Vertex, Weight};
use crate::math::transpose;

fn bfs(matrix: &Vec<Vec<Weight>>, source: Vertex, sink: Vertex, parent: &mut Vec<Option<Vertex>>) -> bool {
    let mut queue: VecDeque<Vertex> = VecDeque::new();
    queue.push_back(source);
    let mut visited = vec![false; matrix.len()];
    visited[source] = true;

    while !queue.is_empty() {
        let u = queue.pop_front().unwrap();
        for (v, capacity) in matrix[u].iter().enumerate() {
            if !visited[v] && *capacity > 0 {
                queue.push_back(v);
                visited[v] = true;
                parent[v] = Some(u)
            }
        }
    }

    return visited[sink]

}

pub fn ford_fulkerson(graph: &dyn Graph) -> (Vertex, Vertex, Weight, Vec<Vec<Weight>>) {
    const INF: i32 = 10000;
    let matrix = graph.adjacency_matrix();
    let mut flow: Vec<Vec<i32>> = vec![vec![0; graph.count_vertex()]; graph.count_vertex()];

    let mut residual_graph = graph.adjacency_matrix();

    let mut parent: Vec<Option<Vertex>> = vec![None; graph.count_vertex()];
    let mut max_flow = 0;

    let transposed = transpose(&graph.adjacency_matrix());

    let mut source = 0;
    let mut sink = 0;

    for i in 0..transposed.len() {
        if transposed[i].iter().all(|x| *x == 0) {
            source = i;
            break;
        }
    }

    for i in 0..residual_graph.len() {
        if residual_graph[i].iter().all(|x| *x == 0) {
            sink = i;
            break;
        }
    }

    while bfs(&residual_graph, source, sink, &mut parent) {
        let mut path_flow = INF;
        let mut s = sink;
        while s != source {
            path_flow = path_flow.min(residual_graph[parent[s].unwrap()][s]);
            s = parent[s].unwrap();
        }

        let mut v = sink;
        while v != source {
            let u = parent[v].unwrap();
            flow[u][v] += path_flow;
            flow[v][u] -= path_flow;

            for i in 0..flow.len() {
                for j in 0..flow.len() {
                    residual_graph[i][j] = matrix[i][j] - flow[i][j];
                }
            }

            v = parent[v].unwrap();
        }
        max_flow += path_flow;
    }

    (source, sink, max_flow, flow)
}
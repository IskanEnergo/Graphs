use std::collections::HashSet;
use crate::algorithms::{ford_fulkerson, is_bipartite};
use crate::graph::{Graph, GraphAdjacencyMatrix, Vertex};

pub fn find_maximal_matching(graph: &dyn Graph) {
    let (_, color) = is_bipartite(graph);
    let color = color.unwrap();

    let mut matrix = graph.adjacency_matrix();

    for i in 0..matrix.len() {
        for j in 0..matrix.len() {
            if matrix[i][j] == 0 {
                continue;
            }

            if color[i] == 0 {
                matrix[j][i] = 0;
            }
        }
    }

    matrix.push(vec![0; matrix.len() + 2]);
    for i in 0..matrix.len() - 1 {
        if color[i] == 0 {
            let len = matrix.len();
            matrix[len - 1][i] = 1;
        }
    }

    matrix.push(vec![0; matrix.len() + 1]);
    for i in 0..matrix.len() - 2 {
        matrix[i].push(0);
        matrix[i].push(0);

        if color[i] == 1 {
            let len = matrix.len();
            matrix[i][len - 1] = 1;
        }
    }

    let g = GraphAdjacencyMatrix::with_matrix(&matrix);

    let (_, _, _, residual_graph) = ford_fulkerson(&g);

    let mut visited: HashSet<Vertex> = HashSet::new();
    let mut matching: Vec<(Vertex, Vertex)> = vec![];

    for line in &residual_graph {
        println!("{:?}", line);
    }

    for i in 0..graph.count_vertex() {
        for j in 0..graph.count_vertex() {
            if graph.is_edge(i, j) {
                if residual_graph[i][j] == 1 && !visited.contains(&i) && !visited.contains(&j) {
                    visited.insert(i);
                    visited.insert(j);
                    matching.push((i, j));
                }
            }
        }
    }

    for m in matching {
        println!("{} - {}", m.0 + 1, m.1 + 1);
    }
}
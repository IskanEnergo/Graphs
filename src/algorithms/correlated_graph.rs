use crate::graph::{Graph, GraphAdjacencyMatrix};

pub fn make_correlated_graph(graph: &dyn Graph) -> Box<dyn Graph> {
    let mut result = graph.adjacency_matrix();

    for i in 0..result.len() {
        for j in 0..result.len() {
            if result[i][j] != result[j][i] {
                if result[i][j] == 0 {
                    result[i][j] = result[j][i]
                } else {
                    result[j][i] = result[i][j]
                }
            }
        }
    }

    Box::new(GraphAdjacencyMatrix::with_matrix(&result))
}
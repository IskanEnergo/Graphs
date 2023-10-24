use crate::graph::{Edge, Graph};

pub fn prim(graph: &dyn Graph) -> Vec<Edge> {
    let mut result: Vec<Edge> = vec![];
    let matrix = graph.adjacency_matrix();
    let mut selected_node = vec![false; matrix.len()];
    selected_node[0] = true;

    for _ in 0..matrix.len() - 1 {
        let mut minimum = i32::MAX;
        let mut a = 0;
        let mut b = 0;

        for m in 0..matrix.len() {
            if selected_node[m] {
                for n in 0..matrix.len() {
                    if !selected_node[n] && matrix[m][n] > 0 {
                        if minimum > matrix[m][n] {
                            minimum = matrix[m][n];
                            a = m;
                            b = n;
                        }
                    }
                }
            }
        }

        result.push((a, b, minimum));
        selected_node[b] = true;
    }

    result
}
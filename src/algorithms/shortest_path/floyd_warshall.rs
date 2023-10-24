use crate::graph::{Graph, Weight};

pub fn floyd_warshall(graph: &dyn Graph) -> Vec<Vec<Weight>> {
    let mut dist = graph.adjacency_matrix();

    for i in 0..dist.len() {
        for j in 0..dist[i].len() {
            if i != j && dist[i][j] == 0 {
                dist[i][j] = i32::MAX;
            }
        }
    }

    let len = dist.len();

    for k in 0..len {
        for i in 0..len {
            for j in 0..len {

                if dist[i][k] == i32::MAX || dist[k][j] == i32::MAX {
                    continue // path throw k does not exists
                }

                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);

                if dist[i][j] == 0 && i != j {
                    dist[i][j] = dist[i][k] + dist[k][j];
                }
            }
        }
    }

    dist
}
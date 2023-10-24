use crate::graph::{Graph, Vertex, Weight};

pub fn bellman_ford(graph: &dyn Graph, s: Vertex) -> (Vec<Weight>, bool) {
    const INF: i32 = 10000;
    let edges = graph.list_of_edges();
    let mut dist: Vec<Weight> = vec![INF; graph.count_vertex()];

    dist[s] = 0;
    for _ in 0..graph.count_vertex() - 1 {
        for edge in &edges {
            if dist[edge.0] + edge.2 < dist[edge.1] {
                dist[edge.1] = dist[edge.0] + edge.2;
            }
        }
    }

    let negative_cycle = edges
        .iter()
        .any(|edge| dist[edge.0] + edge.2 < dist[edge.1]);

    (dist, negative_cycle)
}
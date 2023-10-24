use crate::graph::{Graph, Vertex};

pub fn degree_list(graph: &dyn Graph) -> Vec<Vertex> {
    assert_eq!(graph.is_directed(), false, "Graph must be not directed!");

    (0..graph.count_vertex())
        .into_iter()
        .map(|v| graph.list_of_edges_for_vertex(v).len())
        .collect()
}

/// Returns pair of vectors: (`deg_in[]`, `deg_out[]`)
pub fn degree_list_oriented(graph: &dyn Graph) -> (Vec<Vertex>, Vec<Vertex>) {
    let deg_out: Vec<Vertex> = (0..graph.count_vertex())
        .map(|v| graph.list_of_edges_for_vertex(v).len())
        .collect();

    let list_of_edges = graph.list_of_edges();

    let deg_in: Vec<Vertex> = (0..graph.count_vertex())
        .map(|v| list_of_edges
            .iter()
            .filter(|(_, vj, _)| *vj == v)
            .count()
        )
        .collect();

    (deg_in, deg_out)
}
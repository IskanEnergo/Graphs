use crate::graph::{Edge, Graph, Vertex};

pub fn kruskal(graph: &dyn Graph) -> Vec<Edge> {
    let mut result: Vec<Edge> = vec![];
    let mut graph_edges = graph.list_of_edges();
    graph_edges.sort_by(|e1, e2| e1.2.cmp(&e2.2));

    let mut component_for_vertex: Vec<Vertex> = vec![0; graph.count_vertex()];
    for i in 0..graph.count_vertex() {
        component_for_vertex[i] = i;
    }

    for (u, v, weight) in graph_edges {
        if component_for_vertex[u] == component_for_vertex[v] {
            continue;
        }

        result.push((u, v, weight));
        let old_id = component_for_vertex[v];
        let new_id = component_for_vertex[u];
        for id in component_for_vertex.iter_mut() {
            if *id == old_id {
                *id = new_id;
            }
        }
    }

    result
}
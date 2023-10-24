use crate::algorithms::searchs::{is_graph_strong_connected};
use crate::algorithms::shortest_path::floyd_warshall;
use crate::graph::{Graph, Vertex, Weight};

pub fn eccentricities_list(graph: &dyn Graph) -> Vec<Weight> {
    assert_eq!(is_graph_strong_connected(graph), true);

    floyd_warshall(graph)
        .iter()
        .flat_map(|line| line.iter().copied().reduce(Weight::max))
        .collect()
}

pub fn diameter(graph: &dyn Graph) -> Weight {
    assert_eq!(graph.is_directed(), false);

    eccentricities_list(graph).iter().copied().reduce(Weight::max).unwrap()
}

pub fn radius(graph: &dyn Graph) -> Weight {
    assert_eq!(graph.is_directed(), false);

    eccentricities_list(graph).iter().copied().reduce(Weight::min).unwrap()
}

pub fn centers_list(graph: &dyn Graph) -> Vec<Vertex> {
    assert_eq!(graph.is_directed(), false);

    let r = radius(graph);

    eccentricities_list(graph)
        .iter()
        .enumerate()
        .filter(|(_, e)| **e == r)
        .map(|(v, _)| v)
        .collect()
}

pub fn peripheral_list(graph: &dyn Graph) -> Vec<Vertex> {
    assert_eq!(graph.is_directed(), false);

    let d = diameter(graph);

    eccentricities_list(graph)
        .iter()
        .enumerate()
        .filter(|(_, e)| **e == d)
        .map(|(v, _)| v)
        .collect()
}
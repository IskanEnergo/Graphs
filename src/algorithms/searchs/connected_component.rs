use std::collections::HashSet;
use crate::algorithms::searchs::{bfs};
use crate::graph::{Graph, GraphAdjacencyMatrix, Vertex};
use crate::math::transpose;

fn dfs_utility(graph: &dyn Graph, v: usize, visited: &mut HashSet<Vertex>) -> Vec<Vertex> {

    let mut new_visited: Vec<Vertex> = vec![];

    fn dfs(graph: &dyn Graph, v: usize, visited: &mut HashSet<Vertex>, new_visited: &mut Vec<Vertex>) {
        visited.insert(v);
        new_visited.push(v);

        for i in graph.adjacency_list(v) {
            if !visited.contains(&i) {
                dfs(graph, i, visited, new_visited);
            }
        }
    }

    dfs(graph, v, visited, &mut new_visited);
    new_visited
}

fn fill_stack(graph: &dyn Graph, v: usize, visited: &mut HashSet<Vertex>, stack: &mut Vec<usize>) {
    visited.insert(v);
    for i in graph.adjacency_list(v) {
        if !visited.contains(&i) {
            fill_stack(graph, i, visited, stack);
        }
    }

    stack.push(v);
}

pub fn connected_components(graph: &dyn Graph) -> Vec<HashSet<Vertex>> {
    assert_eq!(graph.is_directed(), false);

    let mut components = vec![];
    let mut unvisited: HashSet<Vertex> = (0..graph.count_vertex()).collect();

    while !unvisited.is_empty() {
        let v = unvisited.iter().next().unwrap();
        let visited = bfs(graph, *v);
        components.push(visited.clone());

        unvisited = unvisited.difference(&visited).map(|v| *v).collect();
    }

    components
}

pub fn strong_connected_components(graph: &dyn Graph) -> Vec<Vec<Vertex>> {
    assert_eq!(graph.is_directed(), true);

    let mut result: Vec<Vec<Vertex>> = vec![];
    let mut stack = vec![];
    let mut visited = HashSet::<Vertex>::new();

    for i in 0..graph.count_vertex() {
        if !visited.contains(&i) {
            fill_stack(graph, i, &mut visited, &mut stack);
        }
    }

    let matrix = transpose(&graph.adjacency_matrix());
    let transpose = GraphAdjacencyMatrix::with_matrix(&matrix);

    visited.clear();
    while let Some(i) = stack.pop() {
        if !visited.contains(&i) {
            let new_component = dfs_utility(&transpose, i, &mut visited);
            result.push(new_component);
        }
    }

    result
}

pub fn weak_connected_components(graph: &dyn Graph) -> Vec<HashSet<Vertex>> {
    assert_eq!(graph.is_directed(), true);

    let adj_matrix = graph.adjacency_matrix();

    let mut matrix = transpose(&adj_matrix);

    for i in 0..matrix.len() {
        for j in 0..matrix.len() {
            matrix[i][j] += adj_matrix[i][j];
            if matrix[i][j] > 0 {
                matrix[i][j] = 1;
            }
        }
    }

    let graph: Box<dyn Graph> = Box::new(GraphAdjacencyMatrix::with_matrix(&matrix));

    connected_components(&*graph)
}
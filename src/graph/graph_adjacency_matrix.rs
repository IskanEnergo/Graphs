use std::io::BufRead;
use crate::graph::{Edge, Weight, Graph, Vertex};

pub struct GraphAdjacencyMatrix {
    matrix: Vec<Vec<Weight>>,
}

impl GraphAdjacencyMatrix {
    pub fn new(reader: impl BufRead) -> Self {
        let arr: Vec<Vec<i32>> = reader.lines()
            .map(|l| l.unwrap().trim().split(char::is_whitespace)
                .map(|number| number.parse().unwrap())
                .collect())
            .collect();
        GraphAdjacencyMatrix { matrix: arr }
    }

    pub fn with_matrix(matrix: &Vec<Vec<Weight>>) -> Self {
        GraphAdjacencyMatrix { matrix: matrix.clone() }
    }
}

impl Graph for GraphAdjacencyMatrix {
    fn weight(&self, vi: Vertex, vj: Vertex) -> Weight {
        self.matrix[vi][vj]
    }

    fn is_edge(&self, vi: Vertex, vj: Vertex) -> bool {
        self.weight(vi, vj) != 0
    }

    fn adjacency_matrix(&self) -> Vec<Vec<Weight>> {
        self.matrix.clone()
    }

    fn adjacency_list(&self, v: Vertex) -> Vec<Vertex> {
        self.matrix[v]
            .iter()
            .enumerate()
            .filter(|(_, weight)| **weight != 0)
            .map(|(u, _)| u)
            .collect()
    }

    fn list_of_edges(&self) -> Vec<Edge> {
        (0..self.count_vertex())
            .flat_map(|v| self.list_of_edges_for_vertex(v))
            .collect()
    }

    fn list_of_edges_for_vertex(&self, v: Vertex) -> Vec<Edge> {
        self.matrix[v]
            .iter()
            .enumerate()
            .map(|(u, weight)| (v, u, *weight))
            .filter(|(_, _, weight)| *weight != 0 )
            .collect::<Vec<Edge>>()
    }

    fn is_directed(&self) -> bool {
        for (i, row) in self.matrix.iter().enumerate() {
            for (j, el) in row.iter().enumerate() {
                if *el != self.matrix[j][i] {
                    return true
                }
            }
        }

        false
    }

    fn count_vertex(&self) -> usize {
        self.matrix[0].len()
    }
}
use std::io::BufRead;
use crate::graph::{Edge, Graph, Vertex, Weight};

pub struct GraphAdjacencyList {
    adjacency_list: Vec<Vec<Vertex>>,
}

impl GraphAdjacencyList {
    pub fn new(reader: impl BufRead) -> Self {
        let mut arr: Vec<Vec<Vertex>> = vec![];
        for l in reader.lines() {
            let l = l.unwrap();

            let s: Vec<&str> = l.trim().split(char::is_whitespace).collect();
            let mut tmp: Vec<Vertex> = vec![];
            for n in s {
                if !n.is_empty() {
                    tmp.push(n.parse::<Vertex>().unwrap() - 1)
                }
            }
            arr.push(tmp);
        }

        GraphAdjacencyList { adjacency_list: arr }
    }
}

impl Graph for GraphAdjacencyList {
    fn weight(&self, vi: Vertex, vj: Vertex) -> Weight {
        match self.is_edge(vi, vj) {
            true => 1,
            false => 0,
        }
    }

    fn is_edge(&self, vi: Vertex, vj: Vertex) -> bool {
        match self.adjacency_list.get(vi) {
            Some(line) => line.iter().find(|u| **u == vj).is_some(),
            None => false
        }
    }

    fn adjacency_matrix(&self) -> Vec<Vec<Weight>> {
        let mut matrix = vec![vec![0; self.count_vertex()]; self.count_vertex()];

        for (i, line) in self.adjacency_list.iter().enumerate() {
            for v in line {
                matrix[i][*v] = 1;
            }
        }

        matrix
    }

    fn adjacency_list(&self, v: Vertex) -> Vec<Vertex> {
        self.adjacency_list.get(v).unwrap_or(&vec![]).clone()
    }

    fn list_of_edges(&self) -> Vec<Edge> {
        self.adjacency_list
            .iter()
            .enumerate()
            .flat_map(|(v, _)| self.list_of_edges_for_vertex(v))
            .collect()
    }

    fn list_of_edges_for_vertex(&self, v: Vertex) -> Vec<Edge> {
        self.adjacency_list(v)
            .iter()
            .map(|u| (v, *u, 1))
            .collect()
    }

    fn is_directed(&self) -> bool {
        !self.list_of_edges()
            .iter()
            .all(|(u, v, _)| self.list_of_edges_for_vertex(*v)
                .iter()
                .any(|(_, v1, _)| *v1 == *u)
            )
    }

    fn count_vertex(&self) -> usize {
        self.adjacency_list.len()
    }
}
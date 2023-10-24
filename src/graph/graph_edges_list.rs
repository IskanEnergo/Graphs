use std::io::BufRead;
use std::collections::HashMap;
use itertools::sorted;
use crate::graph::{Edge, Weight, Graph, Vertex};

pub struct GraphEdgesList {
    vertex_edge_map: HashMap<Vertex, Vec<(Vertex, Weight)>>,
    count_vertex: usize,
}

impl GraphEdgesList {
    pub fn new(reader: impl BufRead) -> Self {
        let mut count = 0;

        let map = reader.lines()
            .map(|l| l.unwrap().trim().split(char::is_whitespace)
                .map(|number| number.parse().unwrap())
                .collect::<Vec<i64>>())
            .map(|el| ((el[0] - 1) as Vertex, (el[1] - 1) as Vertex, (if el.len() == 3 { el[2] } else { 1 }) as Weight))
            .map(|(u, v, weight)| { count = count.max(u.max(v)); (u, v, weight) })
            .fold(
                HashMap::new(),
                |mut map, (v, u, weight)| { map.entry(v).or_insert(vec![]).push((u, weight)); map });


        GraphEdgesList { vertex_edge_map: map, count_vertex: count + 1 }
    }
}

impl Graph for GraphEdgesList {
    fn weight(&self, vi: Vertex, vj: Vertex) -> Weight {
         self.vertex_edge_map
             .get(&vi)
             .unwrap_or(&vec![(0, 0)])
             .iter()
             .find(|(u, _)| *u ==vj)
             .unwrap_or(&(0, 0)).1
    }

    fn is_edge(&self, vi: Vertex, vj: Vertex) -> bool {
        self.weight(vi, vj) != 0
    }

    fn adjacency_matrix(&self) -> Vec<Vec<Weight>> {
        let mut matrix = vec![vec![0; self.count_vertex]; self.count_vertex];

        for v in 0..self.count_vertex {
            for (u, weight) in self.vertex_edge_map.get(&v).unwrap_or(&vec![]) {
                matrix[v][*u] = *weight;
            }
        }

        matrix
    }

    fn adjacency_list(&self, v: Vertex) -> Vec<Vertex> {
        self.vertex_edge_map.get(&v).unwrap_or(&vec![])
            .iter()
            .map(|(u, _)| *u)
            .collect()
    }

    fn list_of_edges(&self) -> Vec<Edge> {
        sorted(self.vertex_edge_map.iter())
            .flat_map(|(v, edge)| edge.iter().map(|(u, weight)| (*v, *u, *weight)))
            .collect()
    }

    fn list_of_edges_for_vertex(&self, v: Vertex) -> Vec<Edge> {
        self.vertex_edge_map.get(&v).unwrap_or(&vec![])
            .iter()
            .map(|(u, weight)| (v, *u, *weight))
            .collect()
    }

    fn is_directed(&self) -> bool {
        !self.vertex_edge_map
            .iter()
            .all(|(v, edge)| edge
                .iter()
                .all(|(u, weight)| *weight == self.weight(*u, *v))
            )
    }

    fn count_vertex(&self) -> usize {
        self.count_vertex
    }
}
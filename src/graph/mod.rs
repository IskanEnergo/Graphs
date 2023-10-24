use std::fs::File;
use std::io::{BufReader};

pub use graph_adjacency_matrix::GraphAdjacencyMatrix;
mod graph_adjacency_matrix;

pub use graph_edges_list::GraphEdgesList;
mod graph_edges_list;

pub use graph_adjacency_list::GraphAdjacencyList;
mod graph_adjacency_list;

pub type Edge = (Vertex, Vertex, Weight);
pub type Vertex = usize;
pub type Weight = i32;

pub use map::{MapCell, Map};
mod map;

pub trait Graph {
    fn weight(&self, vi: Vertex, vj: Vertex) -> Weight;
    fn is_edge(&self, vi: Vertex, vj: Vertex) -> bool;
    fn adjacency_matrix(&self) -> Vec<Vec<Weight>>;
    fn adjacency_list(&self, v: Vertex) -> Vec<Vertex>;
    fn list_of_edges(&self) -> Vec<Edge>;
    fn list_of_edges_for_vertex(&self, v: Vertex) -> Vec<Edge>;
    fn is_directed(&self) -> bool;
    fn count_vertex(&self) -> usize;
}

pub enum InputType {
    EdgesList,
    AdjacencyMatrix,
    AdjacencyList,
}

pub fn create(path: &str, format: InputType) -> Box<dyn Graph> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    match format {
        InputType::EdgesList => Box::new(GraphEdgesList::new(reader)),
        InputType::AdjacencyMatrix => Box::new(GraphAdjacencyMatrix::new(reader)),
        InputType::AdjacencyList => Box::new(GraphAdjacencyList::new(reader))
    }
}
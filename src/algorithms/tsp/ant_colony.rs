use itertools::Itertools;
use rand::prelude::*;
use crate::graph::{Edge, Graph, Vertex, Weight};

const ALPHA: f64 = 1.0;
const BETA: f64 = 3.0;

const Q: f64 = 10.0;
const P: f64 = 0.2;

struct Ant {
    current_city: Vertex,
    visited: Vec<Vertex>,
    path: Vec<Edge>,
}

impl Ant {
    fn start(v: Vertex) -> Self {
        Ant {
            visited: vec![v],
            path: vec![],
            current_city: v,
        }
    }

    fn travel(&mut self, graph: &dyn Graph, pheromone_matrix: &mut Vec<Vec<f64>>) {
        while self.visited.len() < graph.count_vertex() {
            let to = probability_city(graph, &self, &pheromone_matrix, self.current_city);
            self.path.push((self.current_city, to, graph.weight(self.current_city, to)));
            self.visited.push(to);
            self.current_city = to;
        }

        self.path.push(
            (
                *self.visited.last().unwrap(),
                *self.visited.first().unwrap(),
                graph.weight(*self.visited.first().unwrap(), *self.visited.last().unwrap())
            )
        );

        let path_cost = self.path.iter().fold(0, |acc, e| acc + e.2);
        for e in &self.path {
            pheromone_matrix[e.0][e.1] += Q / (path_cost as f64);
        }
    }
}

pub fn ant_colony(graph: &dyn Graph, iterations: Option<usize>) -> (Vec<Edge>, Weight) {
    let mut pheromone_matrix = vec![vec![1.0; graph.count_vertex()]; graph.count_vertex()];

    let (mut shortest_path, mut shortest_cost): (Vec<Edge>, Weight) = (vec![], 1000000);

    for _ in 0..iterations.unwrap_or(100) {
        let mut ants: Vec<Ant> = vec![];

        for v in 0..graph.count_vertex() {
            ants.push(Ant::start(v));
            ants[v].travel(graph, &mut pheromone_matrix);
        }

        let shortest_path_iteration = ants
            .iter()
            .map(|a| (a.path.clone(), a.path.iter().fold(0, |acc, e| acc + e.2)))
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap();

        if shortest_path_iteration.1 < shortest_cost {
            shortest_cost = shortest_path_iteration.1;
            shortest_path = shortest_path_iteration.0;
        }

        for e in graph.list_of_edges() {
            pheromone_matrix[e.0][e.1] = pheromone_matrix[e.0][e.1] * (1.0 - P) + Q / (shortest_path_iteration.1 as f64);
        }
    }

    (shortest_path, shortest_cost)
}

fn probability(graph: &dyn Graph, ant: &Ant, pheromone_matrix: &Vec<Vec<f64>>, current_city: Vertex, to_city: Vertex) -> f64 {
    let unvisited_cities: Vec<Edge> = graph.list_of_edges_for_vertex(current_city)
        .iter()
        .copied()
        .filter(|e| !ant.visited.contains(&e.1))
        .collect();

    let denominator = unvisited_cities
        .iter()
        .fold(0.0, |sum, e|
            sum + (pheromone_matrix[current_city][e.1].powf(ALPHA) as f64) / ((graph.weight(current_city, e.1) as f64).powf(BETA) as f64),
        );

    let numerator = (pheromone_matrix[current_city][to_city].powf(ALPHA) as f64) / ((graph.weight(current_city, to_city) as f64).powf(BETA) as f64);

    numerator / denominator
}

fn probability_city(graph: &dyn Graph, ant: &Ant, pheromone_matrix: &Vec<Vec<f64>>, current_city: Vertex) -> Vertex {
    let mut rng = thread_rng();
    let r: f64 = rng.gen();

    let near_cities_probability: Vec<(Vertex, f64)> = graph.adjacency_list(current_city)
        .iter()
        .copied()
        .filter(|v| !ant.visited.contains(v))
        .map(|u| (u, probability(graph, ant, pheromone_matrix, current_city, u)))
        .sorted_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .collect();

    near_cities_probability
        .iter()
        .find(|(_, p)| r <= *p)
        .unwrap_or(near_cities_probability.last().unwrap()).0
}
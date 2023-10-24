use std::ops::Deref;
use crate::graph::{Edge, Graph, Vertex, Weight};

fn bellman_ford(edges: &mut Vec<Edge>, s: Vertex) -> (Vec<Weight>, bool) {
    const INF: i32 = 10000;
    let mut dist: Vec<Weight> = vec![INF; s + 1];
    dist[s] = 0;

    for i in 0..s {
        edges.push((s, i, 0));
    }

    for _ in 0..s {
        for (src, des, weight) in edges.deref() {
            if (dist[*src] != INF) && (dist[*src] + *weight < dist[*des]) {
                dist[*des] = dist[*src] + *weight;
            }
        }
    }

    let negative_cycle = edges
        .deref()
        .iter()
        .any(|edge| dist[edge.0] + edge.2 < dist[edge.1]);

    dist.remove(dist.len() - 1);

    (dist, negative_cycle)
}

fn min_distance(dist: &Vec<Weight>, marked: &Vec<bool>) -> Vertex {
    const INF: i32 = 10000;

    let (mut minimum, mut min_vertex) = (INF, 0);
    for vertex in 0..dist.len() {
        if minimum > dist[vertex] && !marked[vertex] {
            (minimum, min_vertex) = (dist[vertex], vertex);
        }
    }
    min_vertex
}

fn dijkstra(graph: &Vec<Vec<Weight>>, modified_graph: &Vec<Vec<Weight>>, src: Vertex, ford_dist: &Vec<Weight>) -> Vec<Weight> {
    const INF: i32 = 10000;

    let num_vertices = graph.len();
    let mut marked: Vec<bool> = vec![false; num_vertices];
    let mut dist: Vec<Weight> = vec![INF; num_vertices];

    dist[src] = 0;

    for _ in 0..num_vertices {
        let cur_vertex = min_distance(&dist, &marked);
        marked[cur_vertex] = true;

        for vertex in 0..num_vertices {
            if !marked[vertex] && (dist[vertex] > (dist[cur_vertex] + modified_graph[cur_vertex][vertex])) && graph[cur_vertex][vertex] != 0 {
                dist[vertex] = dist[cur_vertex] + modified_graph[cur_vertex][vertex];
            }
        }
    }

    for i in 0..dist.len() {
        dist[i] += ford_dist[i] - ford_dist[src]
    }

    dist
}

pub fn johnson(graph: &dyn Graph) -> Result<Vec<Vec<Weight>>, &str> {
    let orig_matrix = graph.adjacency_matrix();

    let mut edges = graph.list_of_edges();
    let (modify_weights, negative_cycle) = bellman_ford(&mut edges, graph.count_vertex());

    if negative_cycle {
        return Err("Graph contains a negative cycle.");
    }


    let mut matrix: Vec<Vec<Weight>> = vec![vec![0; graph.count_vertex()]; graph.count_vertex()];

    for i in 0..graph.count_vertex() {
        for j in 0..graph.count_vertex() {
            if  orig_matrix[i][j] != 0 {
                matrix[i][j] = orig_matrix[i][j] + modify_weights[i] - modify_weights[j];
            }
        }
    }

    let mut result: Vec<Vec<Weight>> = vec![];

    for i in 0..graph.count_vertex() {
        result.push(dijkstra(&orig_matrix, &matrix, i, &modify_weights));
    }

    Ok(result)
}
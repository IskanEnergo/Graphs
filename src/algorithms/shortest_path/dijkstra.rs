use crate::graph::{Edge, Graph, Vertex, Weight};

pub fn dijkstra(graph: &dyn Graph, s: Vertex) -> (Vec<Weight>, Vec<Vertex>) {
    let mut dist: Vec<Weight> = vec![i32::MAX; graph.count_vertex()];
    let mut marked: Vec<bool> = vec![false; graph.count_vertex()];
    let mut prev: Vec<Vertex> = vec![0; graph.count_vertex()];

    dist[s] = 0;

    for _ in 0..graph.count_vertex() {
        let mut v: i32 = -1;
        for j in 0..graph.count_vertex() {
            if !marked[j] && (v == -1 || dist[j] < dist[v as usize]) {
                v = j as i32;
            }
        }

        if dist[v as usize] == i32::MAX {
            break;
        }
        marked[v as usize] = true;

        for e in graph.list_of_edges_for_vertex(v as usize) {
            if dist[v as usize] + e.2 < dist[e.1] {
                dist[e.1] = dist[v as usize] + e.2;
                prev[e.1] = v as usize;
            }
        }
    }

    (dist, prev)
}

pub fn chain_dijkstra(graph: &dyn Graph, s: Vertex, d: Vertex) -> Option<(Weight, Vec<Edge>)> {
    let (dist, prev) = dijkstra(graph, s);

    if dist[d] == i32::MAX {
        return None;
    }

    let mut path: Vec<Vertex> = vec![];

    let mut v = d;
    while v != s {
        path.push(v);
        v = prev[v];
    }
    path.push(s);

    let mut path_edges: Vec<Edge> = vec![];
    for i in (1..=path.len() - 1).rev() {
        let u =  path[i];
        let v =  path[i - 1];
        path_edges.push((u, v, graph.weight(u, v)));
    }

    Some((dist[d], path_edges))
}
use std::collections::HashSet;
use crate::graph::{Graph, Vertex};

pub fn cutpoint_and_bridges(graph: &dyn Graph) -> (HashSet<Vertex>, HashSet<(Vertex, Vertex)>) {
    let mut visited: Vec<bool> = vec![false; graph.count_vertex()];
    let mut timer = 0;
    let mut tin: Vec<i32> = vec![0; graph.count_vertex()];
    let mut tup: Vec<i32> = vec![0; graph.count_vertex()];
    let mut parent: Vec<i64> = vec![-1; graph.count_vertex()];

    let mut cutpoints: HashSet<Vertex> = HashSet::new();
    let mut bridges: HashSet<(Vertex, Vertex)> = HashSet::new();

    fn dfs(
        graph: &dyn Graph,
        u: Vertex,
        parent: &mut Vec<i64>,
        visited: &mut Vec<bool>,
        timer: &mut i32,
        tin: &mut Vec<i32>,
        tup: &mut Vec<i32>,
        cutpoints: &mut HashSet<Vertex>,
        bridges: &mut HashSet<(Vertex, Vertex)>
    ) {
        visited[u] = true;

        *timer += 1;
        tin[u] = *timer;
        tup[u] = *timer;

        let mut children = 0;

        for v in graph.adjacency_list(u) {
            if !visited[v] {
                children += 1;
                parent[v] = u as i64;
                dfs(graph, v, parent, visited, timer, tin, tup, cutpoints, bridges);
                tup[u] = tup[u].min(tup[v]);


                if tup[v] > tin[u] {
                    bridges.insert((u, v));
                }

                if parent[u] == -1 && children > 1 {
                    cutpoints.insert(u);
                }

                if parent[u] != -1 && tup[v] >= tin[u] {
                    cutpoints.insert(u);
                }

            } else if (v as i64) != parent[u] { // Нашли уже посещенную вершину - обратное ребро
                tup[u] = tup[u].min(tin[v]);
            }
        }
    }

    dfs(graph, 0, &mut parent, &mut visited, &mut timer, &mut tin, &mut tup, &mut cutpoints, &mut bridges);
    (cutpoints, bridges)
}
use std::collections::VecDeque;
use crate::graph::{Graph, Vertex, Weight};

pub fn levit(graph: &dyn Graph, s: Vertex) -> Vec<Weight> {
    const INF: i32 = 10000;

    let mut dist: Vec<Weight> = vec![INF; graph.count_vertex()];
    dist[s] = 0;

    let mut queue_kind: Vec<i32> = vec![0; graph.count_vertex()];
    queue_kind[s] = 1;


    let mut m0 = VecDeque::<Vertex>::new(); // вершины, расстояние до которых ещё не вычислено.
    let mut m1 = VecDeque::<Vertex>::new(); // вершины, расстояние до которых вычисляется;
    let mut m1_priority = VecDeque::<Vertex>::new(); // приоритетная очередь, вершины, расстояние до которых вычисляется;
    let mut m2 = VecDeque::<Vertex>::new(); // вершины, расстояние до которых уже вычислено (но, возможно, не окончательно);

    m1.push_back(s);

    for i in 0..graph.count_vertex() {
        if i != s {
            m0.push_back(i);
        }
    }

    while !m1.is_empty() || !m1_priority.is_empty() {
        let v = if !m1_priority.is_empty() {
            m1_priority.pop_front().unwrap()
        } else {
            m1.pop_front().unwrap()
        };

        m2.push_back(v);

        for u in graph.list_of_edges_for_vertex(v) {
            if m0.contains(&u.1) {
                dist[u.1] = dist[v] + u.2;
                m0.remove(m0.iter().position(|k| *k == u.1).unwrap());
                m1.push_back(u.1);
            }
            if m1.contains(&u.1) {
                dist[u.1] = dist[u.1].min(dist[v] + u.2);
            }
            if m2.contains(&u.1) && dist[u.1] > dist[v] + u.2 {
                dist[u.1] = dist[v] + u.2;
                m2.remove(m2.iter().position(|k| *k == u.1).unwrap());
                m1_priority.push_back(u.1);
            }
        }
    }

    dist
}
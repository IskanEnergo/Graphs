use crate::graph::{Edge, Graph, Vertex};

fn find_component(component: &mut Vec<usize>, u: Vertex) -> Vertex {
    if component[u] == u {
        u
    } else {
        component[u] = find_component(component, component[u]);
        component[u]
    }
}

fn union(component: &mut Vec<Vertex>, u: Vertex, v: Vertex) {
    let a = find_component(component, u);
    let b = find_component(component, v);
    if a != b {
        component[b] = a;
    }
}

pub fn boruvka(graph: &dyn Graph) -> Vec<Edge> {
    let mut result: Vec<Edge> = vec![];

    let mut component_for_vertex: Vec<Vertex> = vec![0; graph.count_vertex()];
    for i in 0..graph.count_vertex() {
        component_for_vertex[i] = i;
    }

    let mut cheapest: Vec<Edge> = vec![(0, 0, -1); graph.count_vertex()];

    let mut num_trees = graph.count_vertex();

    while num_trees > 1 {
        for (u, v, w) in graph.list_of_edges() {
            let set1 = find_component(&mut component_for_vertex, u);
            let set2 = find_component(&mut component_for_vertex, v);

            if set1 == set2 {
                continue;
            }

            if cheapest[set1].2 == -1 || cheapest[set1].2 > w {
                cheapest[set1] = (u, v, w);
            }

            if cheapest[set2].2 == -1 || cheapest[set2].2 > w {
                cheapest[set2] = (u, v, w);
            }
        }

        for node in &cheapest {
            if node.2 == -1 {
                continue;
            }

            let set1 = find_component(&mut component_for_vertex, node.0);
            let set2 = find_component(&mut component_for_vertex, node.1);

            if set1 != set2 {
                union(&mut component_for_vertex, set1, set2);
                result.push(*node);
                num_trees -= 1;
            }
        }

        for node in cheapest.iter_mut() {
            node.2 = -1;
        }
    }
    result
}
use std::fs::File;
use std::io::Write;
use std::ops::Deref;
use clap::{Parser, Args};
use graph_tool;
use graph_tool::graph::{Edge, InputType, Vertex};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, after_help="Янборисов Искандер М3О-325Бк-21")]
struct Cli {
    #[command(flatten)]
    input_type: Inputs,

    #[command(flatten)]
    algorithm: Algorithms,

    /// Path to output file
    #[arg(short)]
    output_file: String,

    #[arg(short='n')]
    begin_vertex: Vertex,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct Inputs {
    /// The input file is list of edges
    #[arg(short)]
    edge_list: Option<String>,

    /// The input file is adjacency matrix
    #[arg(short)]
    matrix: Option<String>,

    /// The input file is adjacency list
    #[arg(short='l')]
    adjacency_list: Option<String>,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct Algorithms {
    /// Dijkstra
    #[arg(short)]
    dijkstra: bool,

    /// Bellman Ford
    #[arg(short)]
    bellman_ford: bool,

    /// Levit
    #[arg(short='t')]
    levit: bool,
}

fn main() {
    let cli = Cli::parse();

    let (graph_type, input_path) = match (cli.input_type.matrix, cli.input_type.edge_list, cli.input_type.adjacency_list) {
        (Some(v), _, _) => (InputType::AdjacencyMatrix, v),
        (_, Some(v), _) => (InputType::EdgesList, v),
        (_, _, Some(v)) => (InputType::AdjacencyList, v),
        _ => { unreachable!("Provide only one input format") }
    };

    let graph = graph_tool::graph::create(&input_path, graph_type);

    let mut output = File::create(cli.output_file).unwrap();

    let (dist_belman_ford, negative_cycle) = graph_tool::algorithms::shortest_path::bellman_ford(graph.deref(), cli.begin_vertex - 1);
    let is_negative_edge = graph.list_of_edges().iter().any(|e| e.2 < 0);

    if negative_cycle {
        writeln!(&mut output, "Graph contains a negative cycle.").unwrap();
        println!("Graph contains a negative cycle.");
        return;
    }

    if is_negative_edge {
        writeln!(&mut output, "Graph contain edges with negative weight.").unwrap();
        println!("Graph contain edges with negative weight.");
    } else {
        writeln!(&mut output, "Graph does not contain edges with negative weight.").unwrap();
        println!("Graph does not contain edges with negative weight.");
    }

    let dist = match (cli.algorithm.bellman_ford, cli.algorithm.levit, cli.algorithm.dijkstra) {
        (true, _, _) => {
            dist_belman_ford
        },
        (_, true, _) => {
            graph_tool::algorithms::shortest_path::levit(graph.deref(), cli.begin_vertex - 1)
        },
        (_, _, true) => {
            if is_negative_edge {
                panic!("Dijkstra's algorithm cannot be used in a graph with negative edges.")
            }
            graph_tool::algorithms::shortest_path::dijkstra(graph.deref(), cli.begin_vertex - 1).0
        },
        _ => unreachable!(),
    };

    for (i, weight) in dist.iter().enumerate() {
        if i == cli.begin_vertex - 1 {
            continue;
        }
        writeln!(&mut output, "{} - {}: {}", cli.begin_vertex, i + 1, if *weight > 1000 { "∞".to_string() } else { weight.to_string() }).unwrap();
        println!("{} - {}: {}", cli.begin_vertex, i + 1, if *weight > 1000 { "∞".to_string() } else { weight.to_string() });
    }
}
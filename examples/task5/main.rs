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

    /// Path to output file
    #[arg(short)]
    output_file: String,

    #[arg(short='n')]
    begin_vertex: Vertex,


    #[arg(short='d')]
    end_vertex: Vertex,
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

    let res = graph_tool::algorithms::shortest_path::chain_dijkstra(graph.deref(), cli.begin_vertex - 1, cli.end_vertex - 1);

    if let Some(r) = res {
        writeln!(&mut output, "Shortest path length between {} and {} vertices: {}", cli.begin_vertex, cli.end_vertex, r.0);
        println!("Shortest path length between {} and {} vertices: {}", cli.begin_vertex, cli.end_vertex, r.0);

        writeln!(&mut output, "Path:");
        println!("Path:");
        let path: Vec<Edge> = r.1.iter().map(|e| (e.0 + 1, e.1 + 1, e.2)).collect();
        writeln!(&mut output, "{:?}", path);
        println!("{:?}", path);
    } else {
        writeln!(&mut output, "There is no path between the vertices {} and {}.", cli.begin_vertex, cli.end_vertex);
        println!("There is no path between the vertices {} and {}.", cli.begin_vertex, cli.end_vertex);
    }
}
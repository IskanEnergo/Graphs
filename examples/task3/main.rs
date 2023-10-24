use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::ops::Deref;
use clap::{Parser, Args};
use graph_tool;
use graph_tool::graph::{InputType, Vertex};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, after_help="Янборисов Искандер М3О-325Бк-21")]
struct Cli {
    #[command(flatten)]
    input_type: Inputs,

    /// Path to output file
    #[arg(short)]
    output_file: String,
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

    let (cutpoints, bridges) = graph_tool::algorithms::searchs::cutpoint_and_bridges(graph.deref());
    let cutpoints: HashSet<Vertex> = cutpoints.iter().map(|v| *v + 1).collect();
    let bridges: HashSet<(Vertex, Vertex)> = bridges.iter().map(|(v, u)| (*v + 1, *u + 1)).collect();

    writeln!(&mut output, "{:?}", cutpoints).unwrap();
    writeln!(&mut output, "{:?}", bridges).unwrap();

    println!("{:?}", cutpoints);
    println!("{:?}", bridges);
}
use clap::{Parser, Args};
use std::fs::File;
use std::io::Write;
use graph_tool;
use graph_tool::algorithms::{degree_list, degree_list_oriented};
use graph_tool::algorithms::searchs::is_graph_strong_connected;
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

    if graph.is_directed() {
        let (deg_in, deg_out) = degree_list_oriented(&*graph);
        writeln!(&mut output, "deg+ = {:?}", deg_in).unwrap();
        writeln!(&mut output, "deg- = {:?}", deg_out).unwrap();
        println!("deg+ = {:?}", deg_in);
        println!("deg- = {:?}", deg_out);

    } else {
        let deg = degree_list(&*graph);
        writeln!(&mut output, "deg = {:?}", deg).unwrap();
        println!("deg = {:?}", deg);
    }

    writeln!(&mut output, "Distances:").unwrap();
    println!("Distances:");
    let matrix = graph_tool::algorithms::shortest_path::floyd_warshall(&*graph);
    for line in matrix {
        writeln!(&mut output, "{:?}", line).unwrap();
        println!("{:?}", line);
    }

    if !is_graph_strong_connected(&*graph) {
        return;
    }

    let eccentricities = graph_tool::algorithms::eccentricities_list(&*graph);
    writeln!(&mut output, "Eccentricity: {:?}", eccentricities).unwrap();
    println!("Eccentricity: {:?}", eccentricities);

    if graph.is_directed() {
        return;
    }

    let d = graph_tool::algorithms::diameter(&*graph);
    writeln!(&mut output, "D = {}", d).unwrap();
    println!("D = {}", d);

    let r = graph_tool::algorithms::radius(&*graph);
    writeln!(&mut output, "R = {}", r).unwrap();
    println!("R = {}", r);

    let z = graph_tool::algorithms::centers_list(&*graph)
        .iter()
        .map(|v| v + 1)
        .collect::<Vec<Vertex>>();
    writeln!(&mut output, "Z = {:?}", z).unwrap();
    println!("Z = {:?}", z);

    let p = graph_tool::algorithms::peripheral_list(&*graph)
        .iter()
        .map(|v| v + 1)
        .collect::<Vec<Vertex>>();
    writeln!(&mut output, "P = {:?}", p).unwrap();
    println!("P = {:?}", p);
}
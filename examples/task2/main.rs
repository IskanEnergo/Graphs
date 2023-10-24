use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use clap::{Parser, Args};
use itertools::sorted;
use graph_tool;
use graph_tool::algorithms::searchs::connected_components;
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
        let strong_components: Vec<Vec<Vertex>> = graph_tool::algorithms::searchs::strong_connected_components(&*graph)
            .iter()
            .map(|l| l
                .iter()
                .map(|v| *v + 1)
                .collect())
            .collect();
        let weak_components: Vec<HashSet<Vertex>> = graph_tool::algorithms::searchs::weak_connected_components(&*graph)
            .iter()
            .map(|l| l
                .iter()
                .map(|v| *v + 1)
                .collect())
            .collect();

        match weak_components.len() {
            1 => {
                writeln!(&mut output, "Digraph is connected.").unwrap();
                println!("Digraph is connected.");
            },
            _ => {
                writeln!(&mut output, "Digraph is not connected.").unwrap();
                println!("Digraph is not connected.");
            },
        }

        writeln!(&mut output, "Connected components:").unwrap();
        writeln!(&mut output, "{:?}", weak_components).unwrap();
        println!("Connected components:");
        println!("{:?}", weak_components);

        match strong_components.len() {
            1 => {
                writeln!(&mut output, "Digraph is strongly connected.").unwrap();
                println!("Digraph is strongly connected.");
            },
            _ => {
                writeln!(&mut output, "Digraph is weakly connected and contains {} strongly connected components.", strong_components.len()).unwrap();
                println!("Digraph is weakly connected and contains {} strongly connected components.", strong_components.len())
            },
        }

        writeln!(&mut output, "Strongly connected components:").unwrap();
        writeln!(&mut output, "{:?}", strong_components).unwrap();
        println!("Strongly connected components:");
        println!("{:?}", strong_components);
    } else {
        let components = connected_components(&*graph);

        let components: Vec<Vec<Vertex>> = components.iter().map(|component| component
            .iter()
            .map(|v| *v + 1)
            .collect::<Vec<Vertex>>()
        )
            .collect();

        let components:Vec<Vec<Vertex>> = components
            .iter()
            .map(|vs| sorted(vs.iter())
                .map(|v| *v)
                .collect())
            .collect();

        match components.len() {
            1 => {
                writeln!(&mut output, "Graph is connected.").unwrap();
                println!("Graph is connected.")
            },
            _ => {
                writeln!(&mut output, "Graph is not connected.").unwrap();
                println!("Graph is not connected.")
            },
        }

        writeln!(&mut output, "Connected components:").unwrap();
        writeln!(&mut output, "{:?}", components).unwrap();
        println!("Connected components:");
        println!("{:?}", components);
    }
}
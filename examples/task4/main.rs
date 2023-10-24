use std::fs::File;
use std::io::Write;
use std::ops::Deref;
use std::time::Instant;
use clap::{Parser, Args};
use graph_tool;
use graph_tool::algorithms::make_correlated_graph;
use graph_tool::graph::{Edge, Graph, InputType};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, after_help="Янборисов Искандер М3О-325Бк-21")]
struct Cli {
    #[command(flatten)]
    input_type: Inputs,

    /// Algorithm
    #[command(flatten)]
    algorithm: Algorithms,

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

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct Algorithms {
    /// Kruskal
    #[arg(short)]
    kruskal: bool,

    /// Prim
    #[arg(short)]
    prim: bool,

    /// Boruvka
    #[arg(short)]
    boruvka : bool,

    /// Boruvka
    #[arg(short='s')]
    all : bool,
}

fn bench(output: &mut File, graph: &dyn Graph, f: fn(&dyn Graph) -> Vec<Edge>, alg_name: &str) {
    let now = Instant::now();
    f(graph);
    let elapsed = now.elapsed();
    writeln!(output, "{}: elapsed {} ms", alg_name, elapsed.as_millis()).unwrap();
    println!("{}: elapsed {} ms", alg_name, elapsed.as_millis());
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

    let graph: Box<dyn Graph> = match graph.is_directed() {
        true => make_correlated_graph(graph.deref()),
        false => graph,
    };

    let mut output = File::create(cli.output_file).unwrap();

    if cli.algorithm.all {
        bench(&mut output, graph.deref(), graph_tool::algorithms::minimum_spanning_tree::kruskal, "Kruskal");
        bench(&mut output, graph.deref(), graph_tool::algorithms::minimum_spanning_tree::prim, "Prim");
        bench(&mut output, graph.deref(), graph_tool::algorithms::minimum_spanning_tree::boruvka, "Boruvka");
        return;
    }

    let tree = match (cli.algorithm.kruskal, cli.algorithm.prim, cli.algorithm.boruvka) {
        (true, _, _) => graph_tool::algorithms::minimum_spanning_tree::kruskal(graph.deref()),
        (_, true, _) => graph_tool::algorithms::minimum_spanning_tree::prim(graph.deref()),
        (_, _, true) => graph_tool::algorithms::minimum_spanning_tree::boruvka(graph.deref()),
        _ => unreachable!(),
    };

    let tree: Vec<Edge> = tree.iter().map(|(u, v, w)| (*u + 1, *v + 1, *w)).collect();

    let sum = tree.iter().fold(0, |acc, e| acc + e.2);

    writeln!(&mut output, "Minimum spanning tree:").unwrap();
    println!("Minimum spanning tree:");

    writeln!(&mut output, "{:?}", tree).unwrap();
    println!("{:?}", tree);

    writeln!(&mut output, "Weight of spanning tree: {}", sum).unwrap();
    println!("Weight of spanning tree: {}", sum);
}
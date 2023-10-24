use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::{Parser};
use clap::builder::Str;
use graph_tool;
use graph_tool::algorithms::shortest_path::Heuristic;
use graph_tool::graph::{Map};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, after_help = "Янборисов Искандер М3О-325Бк-21")]
struct Cli {
    #[arg(short = 'f')]
    input_file: String,

    /// Path to output file
    #[arg(short)]
    output_file: Option<String>,

    #[arg()]
    input: String,
}

fn path_cost(map: &Map, path: &Vec<(usize, usize)>) -> isize {
    let mut cost = 0 as isize;
    for i in 0..path.len() - 1 {
        let current = path[i];
        let next = path[i + 1];
        let height_diff = (map.get_by_tuple(current).height - map.get_by_tuple(next).height).abs() as isize;
        cost += 1 + height_diff;
    }
    cost
}

/// Манхэттенское расстояние
fn heuristic_manhattan(a: (usize, usize), b: (usize, usize)) -> usize {
    ((b.0 as isize - a.0 as isize).abs() + (b.1 as isize - a.1 as isize).abs()) as usize
}

/// Расстояние Чебышева
fn heuristic_chebyshev(a: (usize, usize), b: (usize, usize)) -> usize {
    (b.0 as isize - a.0 as isize).abs().max((b.1 as isize - a.1 as isize).abs()) as usize
}

/// Евклидово расстояние
fn heuristic_euclidean(a: (usize, usize), b: (usize, usize)) -> usize {
    (((b.0 as isize - a.0 as isize).pow(2) + (b.1 as isize - a.1 as isize).pow(2)) as f64).sqrt() as usize
}

/// Эвристика отсутствует
fn heuristic_dijkstra(a: (usize, usize), b: (usize, usize)) -> usize {
    0
}

fn main() {
    let cli = Cli::parse();

    let file = File::open(cli.input_file).unwrap();
    let reader = BufReader::new(file);

    let map: Vec<Vec<isize>> = reader.lines()
        .map(|l| l.unwrap().trim().split(char::is_whitespace)
            .map(|number| number.parse().unwrap())
            .collect())
        .collect();

    let map = graph_tool::graph::Map::with_matrix(&map);

    let input: Vec<usize> = cli.input
        .split(char::is_whitespace)
        .map(|s| s.parse().unwrap())
        .collect();

    let start = (input[0], input[1]);
    let goal = (input[2], input[3]);

    let start = map.get_by_tuple(start);
    let goal = map.get_by_tuple(goal);

    let mut heuristics: HashMap<String, Heuristic> = HashMap::new();
    heuristics.insert("Manhattan".to_string(), heuristic_manhattan);
    heuristics.insert("Chebyshev".to_string(), heuristic_chebyshev);
    heuristics.insert("Euclidean".to_string(), heuristic_euclidean);
    heuristics.insert("Dijkstra".to_string(), heuristic_dijkstra);

    if let Some((path, _)) = graph_tool::algorithms::shortest_path::astar(&map, &start, &goal, heuristic_dijkstra) {
        println!("Path found: {:?}", path);
        let cost = path_cost(&map, &path);
        println!("Path cost: {}", cost);
    } else {
        println!("No path found");
    }

    for heuristic in heuristics {
        if let Some((_, count)) = graph_tool::algorithms::shortest_path::astar(&map, &start, &goal, heuristic.1) {
            let percent = (count * 100) as f64 / (map.height() * map.width()) as f64;
            println!("{}: {} ({:.2}%)", heuristic.0, count, percent);
        } else {
            println!("Error for {} heuristic", heuristic.0);
        }
    }

}
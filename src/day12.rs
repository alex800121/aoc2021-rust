use project_root::get_project_root;
use petgraph::{Graph, graph::UnGraph, data::Build};

pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let mut g: UnGraph<&str, ()> = Graph::new_undirected();
    let mut nodes = Vec::new();
    for s in input.lines() {
        if let Some((a, b)) = s.split_once('-') {
            let mut x;
            let mut y;
        }
    }
}

use itertools::Itertools;
use project_root::get_project_root;
use petgraph::{Graph, graph::UnGraph, data::Build};


#[derive(Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
enum Node<'a> {
    Start,
    End,
    Upper(&'a str),
    Lower(&'a str),
}

impl<'a> Node<'a> {
    fn from_str(s: &'a str) -> Node<'a> {
        use Node::*;
        match s {
            "start" => Start,
            "end" => End,
            s => {
                let mut e = s.clone().chars();
                if let Some(c) = e.next() {
                    if c.is_lowercase() {
                        Lower(s)
                    } else {
                        Upper(s)
                    }
                } else {
                    unreachable!()
                }
            }
        }
    }
}
fn insert_node(i: &mut u32, nodes: &mut Vec<Node>, node: &str) -> u32 {
    let node = Node::from_str(node);
    unimplemented!()
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let mut g: UnGraph<&str, ()> = Graph::new_undirected();
    let mut nodes = Vec::new();
    let mut i = 0;
    for s in input.lines() {
        if let Some((a, b)) = s.split_once('-') {
            let a = Node::from_str(a);
            let b = Node::from_str(b);
            let mut ia = 0;
            let mut ib = 0;
            if let Some((n, _)) = nodes.iter().find_position(|x| **x == a) {
                ia = n;
            } else {
                nodes.push(a);
                ia = i;
                i += 1;
            }
            if let Some((n, _)) = nodes.iter().find_position(|x| **x == b) {
                ib = n;
            } else {
                nodes.push(b);
                ib = i;
                i += 1;
            }
        }
    }
}

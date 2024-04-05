use std::collections::{BTreeSet, BTreeMap};

use itertools::Itertools;
use project_root::get_project_root;
use petgraph::{Graph, graph::{UnGraph, NodeIndex}};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct State {
    current_node: u16,
    visited_small: u16,
    visited_once: Option<u16>,
}
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
                let mut e = s.chars();
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

pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let mut nodes: BTreeMap<Node, NodeIndex> = BTreeMap::new();
    let mut g: UnGraph<Node, ()> = Graph::new_undirected();
    for s in input.lines() {
        if let Some((a, b)) = s.split_once('-') {
            let a = Node::from_str(a);
            let ia = if let Some(n) = nodes.get(&a) {
                *n
            } else {
                let ia = g.add_node(a);
                nodes.insert(a, ia);
                ia
            };
            let b = Node::from_str(b);
            let ib = if let Some(n) = nodes.get(&b) {
                *n
            } else {
                let ib = g.add_node(b);
                nodes.insert(b, ib);
                ib
            };
            g.add_edge(ia, ib, ());
        }
    }
    dbg!(&g);
}

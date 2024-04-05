use std::collections::BTreeMap;

use itertools::Itertools;
use petgraph::{
    graph::{NodeIndex, UnGraph},
    visit::IntoNodeReferences,
    Graph,
};
use project_root::get_project_root;

#[derive(Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct State {
    current_node: u32,
    visited: u16,
    visited_once: Option<u32>,
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

// fn bfs(g: &UnGraph<Node, ()>, state: State) -> Vec<State> {
fn bfs(g: &UnGraph<Node, ()>, state: State) -> u32 {
    use Node::*;
    // let mut start = BTreeMap::from([(state, 1)]);
    let mut start = vec![state];
    let mut acc = 0;
    while !start.is_empty() {
        let mut new_start = Vec::new();
        // let mut new_start = BTreeMap::new();
        for s in start.into_iter() {
            // for (s, n) in start.into_iter() {
            // dbg!(s, g.neighbors_undirected(s.current_node.into()).collect_vec());
            for next in g.neighbors_undirected(s.current_node.into()) {
                match g.node_weight(next) {
                    Some(End) => {
                        acc += 1;
                        // acc_v.push(s);
                    }
                    // Some(End) => acc += n,
                    Some(Upper(_)) => {
                        let next_s = State {
                            current_node: next.index() as u32,
                            visited: s.visited | (1 << next.index()),
                            ..s
                        };
                        // if let Some(x) = new_start.get_mut(&next_s) {
                        //     *x += n;
                        // } else {
                        //     new_start.insert(next_s, n);
                        // }
                        new_start.push(next_s);
                    }
                    Some(Lower(_)) if (s.visited >> next.index()) & 1 == 0 => {
                        let next_s = State {
                            current_node: next.index() as u32,
                            visited: s.visited | (1 << next.index()),
                            ..s
                        };
                        // if let Some(x) = new_start.get_mut(&next_s) {
                        //     *x += n;
                        // } else {
                        //     new_start.insert(next_s, n);
                        // }
                        new_start.push(next_s);
                    }
                    Some(Lower(_))
                        if (s.visited >> next.index()) & 1 == 1 && s.visited_once.is_none() =>
                    {
                        let next_s = State {
                            current_node: next.index() as u32,
                            visited_once: Some(next.index() as u32),
                            ..s
                        };
                        // if let Some(x) = new_start.get_mut(&next_s) {
                        //     *x += n;
                        // } else {
                        //     new_start.insert(next_s, n);
                        // }
                        new_start.push(next_s);
                    }
                    _ => {}
                }
            }
        }
        start = new_start;
    }
    acc
    // acc_v
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        // "{}/input/test{:02}.txt",
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
    // dbg!(&g);
    dbg!(bfs(
        &g,
        State {
            current_node: g
                .node_references()
                .find(|x| *x.1 == Node::Start)
                .unwrap()
                .0
                .index() as u32,
            visited: 0,
            visited_once: Some(0),
        }
    ));
    dbg!(bfs(
        &g,
        State {
            current_node: g
                .node_references()
                .find(|x| *x.1 == Node::Start)
                .unwrap()
                .0
                .index() as u32,
            visited: 0,
            visited_once: None,
        }
    ));
}

use std::collections::BTreeMap;

use itertools::Itertools;
use project_root::get_project_root;

type State = BTreeMap<[char; 2], u64>;
type Map = BTreeMap<[char; 2], [[char; 2]; 2]>;

fn polymerize(state: &mut State, ins: &Map) {
    let mut new_state = BTreeMap::new();
    while let Some((k, n)) = state.pop_first() {
        for a in ins.get(&k).unwrap() {
            if let Some(m) = new_state.get_mut(a) {
                *m += n;
            } else {
                new_state.insert(*a, n);
            }
        }
    }
    *state = new_state;
}
fn calc_ans(state: &State, start: char, end: char) -> u64 {
    let mut chars = BTreeMap::new();
    for (ks, n) in state.iter() {
        for k in ks {
            if let Some(m) = chars.get_mut(k) {
                *m += *n;
            } else {
                chars.insert(k, *n);
            }
        }
    }
    if let Some(n) = chars.get_mut(&start) {
        *n += 1;
    }
    if let Some(n) = chars.get_mut(&end) {
        *n += 1;
    }
    match chars.values().minmax() {
        itertools::MinMaxResult::NoElements => 0,
        itertools::MinMaxResult::OneElement(_) => 0,
        itertools::MinMaxResult::MinMax(a, b) => (b - a) / 2,
    }
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let ((mut state, start, end), ins) = {
        let (input, ins) = input.split_once('\n').unwrap();
        let mut i = BTreeMap::new();
        for ins in ins.lines() {
            if let Some((a, b)) = ins.split_once(" -> ") {
                let a: [char; 2]= a.chars().collect_vec().try_into().unwrap();
                let c: [char; 2] = [a[0], b.chars().next().unwrap()];
                let d: [char; 2] = [b.chars().next().unwrap(), a[1]];
                i.insert(a, [c, d]);
            }
        }
        let mut y = input.chars();
        let (a, b) = (y.next().unwrap(), y.last().unwrap());
        let mut y = input.chars().peekable();
        let mut x = BTreeMap::new();
        while let (Some(a), Some(b)) = (y.next(), y.peek()) {
            if let Some(n) = x.get_mut(&[a, *b]) {
                *n += 1;
            } else {
                x.insert([a, *b], 1u64);
            }
        }
        ((x, a, b), i)
    };
    for _ in 0..10 {
        polymerize(&mut state, &ins)
    }
    println!("day14a: {}", calc_ans(&state, start, end));
    for _ in 0..(40 - 10) {
        polymerize(&mut state, &ins)
    }
    println!("day14b: {}", calc_ans(&state, start, end));
}

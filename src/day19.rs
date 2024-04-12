use std::{
    array::from_fn,
    collections::{BTreeMap, BTreeSet},
};

use itertools::Itertools;
use project_root::get_project_root;
use std::mem::take;

type Index = (i32, i32, i32);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Scanner {
    position: Index,
    beacons: Vec<Index>,
}
fn read_scanner(s: &str) -> Scanner {
    let mut b = Vec::new();
    let mut s = s.lines();
    s.next();
    for i in s {
        let [x, y, z] = i
            .splitn(3, ',')
            .filter_map(|x| x.parse::<i32>().ok())
            .collect_vec()
            .try_into()
            .unwrap();
        b.push((x, y, z));
    }
    Scanner {
        position: (0, 0, 0),
        beacons: b,
    }
}
fn turn_index(i: Index) -> [Index; 24] {
    let mut v = Vec::new();
    let mut j = i;
    for _ in 0..4 {
        j = (j.0, -j.2, j.1);
        v.push(j);
    }
    for n in 0..4 {
        j = (-j.2, j.1, j.0);
        if n % 2 == 0 {
            v.push(j);
        }
    }
    v.into_iter()
        .flat_map(|mut j| {
            let mut v = Vec::new();
            for _ in 0..4 {
                j = (-j.1, j.0, j.2);
                v.push(j);
            }
            v
        })
        .collect_vec()
        .try_into()
        .unwrap()
}
fn turn_scanner(s: &Scanner) -> [Scanner; 24] {
    let pos = turn_index(s.position);
    let mut beacons: [Vec<Index>; 24] = from_fn(|_| Vec::new());
    for x in s.beacons.iter() {
        for (n, y) in turn_index(*x).into_iter().enumerate() {
            beacons[n].push(y);
        }
    }
    from_fn(|n| Scanner {
        position: pos[n],
        beacons: beacons[n].to_owned(),
    })
}

fn scanner_id(s: &Scanner) -> BTreeMap<[i32; 3], usize> {
    let mut b = BTreeMap::new();
    for xs in s.beacons.iter().combinations(2) {
        let (Some(&&x), Some(&&y)) = (xs.first(), xs.get(1)) else { continue };
        let mut v = [x.0 - y.0, x.1 - y.1, x.2 - y.2].map(|x| x.abs());
        v.sort();
        if let Some(n) = b.get_mut(&v) {
            *n += 1;
        } else {
            b.insert(v, 1);
        }
    }
    b
}

fn compare_scanner(s1: &Scanner, s2: &mut Scanner) -> bool {
    let s1_id = scanner_id(s1);
    let s2_id = scanner_id(s2);
    let mut acc = 0;
    for s in s1_id.iter() {
        if let Some(n) = s2_id.get(s.0) {
            acc += n.min(s.1);
        }
    }
    if acc < (12 * 11) / 2 {
        return false;
    }
    let mut s_final = s2.clone();
    let mut b_final = false;
    // let mut m = 0;
    'a: for mut s in turn_scanner(s2) {
        for (x0, y0, z0) in s1.beacons.iter() {
            for (x1, y1, z1) in s.beacons.clone().into_iter() {
                let (x, y, z) = (x1 - x0, y1 - y0, z1 - z0);
                s.position = (s.position.0 - x, s.position.1 - y, s.position.2 - z);
                s.beacons = s
                    .beacons
                    .into_iter()
                    .map(|(a, b, c)| (a - x, b - y, c - z))
                    .collect();
                // let a = BTreeSet::from_iter(s1.beacons.iter())
                //     .intersection(&BTreeSet::from_iter(s.beacons.iter()))
                //     .count();
                // if a > m {
                //     m = a;
                // };
                if BTreeSet::from_iter(s1.beacons.iter())
                    .intersection(&BTreeSet::from_iter(s.beacons.iter()))
                    .count()
                    >= 12
                {
                    s_final = s;
                    b_final = true;
                    break 'a;
                }
                s.position = (s.position.0 + x, s.position.1 + y, s.position.2 + z);
                s.beacons = s
                    .beacons
                    .into_iter()
                    .map(|(a, b, c)| (a + x, b + y, c + z))
                    .collect();
            }
        }
    }
    *s2 = s_final;
    b_final
}
fn solve_all(mut input: Vec<Scanner>) -> Vec<Scanner> {
    let mut output = Vec::new();
    let mut acc = Vec::from_iter(input.pop());
    while !acc.is_empty() {
        let mut new_acc = Vec::new();
        for ref_s in acc.drain(..) {
            let mut new_input = Vec::new();
            for mut i in input.drain(..) {
                if compare_scanner(&ref_s, &mut i) {
                    new_acc.push(i);
                } else {
                    new_input.push(i);
                }
            }
            input = take(&mut new_input);
            output.push(ref_s);
        }
        acc = take(&mut new_acc);
    }
    output
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        // "{}/input/test{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input = input.split("\n\n").map(read_scanner).collect_vec();
    let output = solve_all(input);
    println!("day19a: {}", output.iter().flat_map(|x| x.beacons.iter()).collect::<BTreeSet<_>>().len());
    println!("day19b: {}", output.iter().map(|x| x.position).permutations(2).filter_map(|x| {
        let (x0, y0, z0) = x.first()?;
        let (x1, y1, z1) = x.get(1)?;
        Some((x1 - x0).abs() + (y1 - y0).abs() + (z1 - z0).abs())
    }).max().unwrap());
}

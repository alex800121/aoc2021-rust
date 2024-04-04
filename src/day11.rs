use std::collections::BTreeSet;

use itertools::Itertools;
use project_root::get_project_root;

type Octopus = [[u8; 10]; 10];
type Index = (isize, isize);

fn inc(o: &mut Octopus) {
    o.iter_mut().for_each(|v| {
        v.iter_mut().for_each(|x| {
            *x += 1;
        })
    });
}

fn surroudings(p: Index) -> [Index; 8] {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .map(|(a, b)| (p.0 + a, p.1 + b))
}
fn flash(o: &mut Octopus) -> u32 {
    inc(o);
    let mut flashing: BTreeSet<Index> = BTreeSet::new();
    for (y, v) in o.iter().enumerate() {
        for (x, i) in v.iter().enumerate() {
            if *i >= 10 {
                flashing.insert((x as isize, y as isize));
            }
        }
    }
    while !flashing.is_empty() {
        // dbg!(&flashing);
        let mut new_flash = BTreeSet::new();
        for p in flashing.into_iter() {
            for (x, y) in surroudings(p).into_iter() {
                if let Some(n) = o.get_mut(y as usize).and_then(|v| v.get_mut(x as usize)) {
                    if *n < 10 {
                        *n += 1;
                        if *n >= 10 {
                            new_flash.insert((x, y));
                        }
                    }
                }
            }
        }
        flashing = new_flash;
    }
    let mut acc = 0;
    for v in o.iter_mut() {
        for i in v.iter_mut() {
            if *i >= 10 {
                *i = 0;
                acc += 1
            }
        }
    }
    acc
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let mut o: Octopus = input
        .lines()
        .filter_map(|v| {
            v.chars()
                .filter_map(|c| c.to_digit(10).map(|n| n as u8))
                .collect_vec()
                .try_into()
                .ok()
        })
        .collect_vec()
        .try_into()
        .unwrap();
    let mut acc = 0;
    let mut o0 = o.clone();
    for _ in 0..100 {
        acc += flash(&mut o0);
    }
    println!("day11a: {}", acc);
    let mut acc = 0;
    while !o.iter().all(|v| v.iter().all(|x| *x == 0)) {
        flash(&mut o);
        acc += 1;
    }
    println!("day11b: {}", acc);
}

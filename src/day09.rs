use itertools::Itertools;
use project_root::get_project_root;
use std::collections::BTreeSet;

type Map = [BTreeSet<Point>; 10];
type Point = (isize, isize);
fn adjacent(p: Point) -> [Point; 4] {
    [(0, 1), (0, -1), (1, 0), (-1, 0)].map(|(a, b)| (p.0 + a, p.1 + b))
}
fn is_low(m: Map) -> BTreeSet<Point> {
    let mut is_low = BTreeSet::new();
    let mut is_not_low = BTreeSet::new();
    for mut level in m.into_iter() {
        'a: loop {
            let (x, mut y): (BTreeSet<_>, BTreeSet<_>) = level.into_iter().partition(|p| {
                adjacent(*p)
                    .iter()
                    .all(|p| !is_low.contains(p) && !is_not_low.contains(p))
            });
            level = x;
            if y.is_empty() {
                break 'a;
            };
            is_not_low.append(&mut y);
        }
        is_low.append(&mut level);
    }
    is_low
}
fn bfs(v: &[[u32; 100]; 100], start: BTreeSet<Point>) -> BTreeSet<Point> {
    let mut front = start;
    let mut acc = BTreeSet::new();
    while !front.is_empty() {
        let mut next_front = front
            .iter()
            .flat_map(|x| {
                adjacent(*x).into_iter().filter(|p| {
                    !acc.contains(p)
                        && p.0 >= 0
                        && p.1 >= 0
                        && p.0 < 100
                        && p.1 < 100
                        && v[p.1 as usize][p.0 as usize] != 9
                })
            })
            .collect();
        acc.append(&mut front);
        front.append(&mut next_front);
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
    let init = BTreeSet::new();
    let mut v = [[0; 100]; 100];
    let mut m: Map = std::array::from_fn(|_| init.clone());
    input.lines().enumerate().for_each(|(iy, y)| {
        y.chars().enumerate().for_each(|(ix, c)| {
            if let Some(c) = c.to_digit(10) {
                m[c as usize].insert((ix as isize, iy as isize));
                v[iy][ix] = c;
            }
        })
    });
    let low_points = is_low(m);
    println!(
        "day9a: {}",
        low_points
            .iter()
            .map(|p| v[p.1 as usize][p.0 as usize] + 1)
            .sum::<u32>()
    );
    let basins = low_points.into_iter().map(|p| bfs(&v, BTreeSet::from([p]))).collect::<BTreeSet<_>>();
    let mut basins = basins.iter().map(|x| x.len()).collect_vec();
    basins.sort();
    basins.reverse();
    basins.truncate(3);
    println!("day9b: {}", basins.iter().product::<usize>());
}

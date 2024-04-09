use std::array::from_fn;

use itertools::Itertools;
use pathfinding::prelude::astar;
use project_root::get_project_root;

const N: usize = 100;
const BIGN: usize = 500;
type Map = [[isize; N]; N];
type BigMap = [[isize; BIGN]; BIGN];
type Index = (isize, isize);
fn adjacent(p: Index) -> [Index; 4] {
    [(0, 1), (0, -1), (-1, 0), (1, 0)].map(|(a, b)| (p.0 + a, p.1 + b))
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let m: Map = input
        .lines()
        .filter_map(|l| {
            l.chars()
                .filter_map(|x| x.to_digit(10).map(|x| x as isize))
                .collect_vec()
                .try_into()
                .ok()
        })
        .collect_vec()
        .try_into()
        .unwrap();
    println!(
        "day15a: {}",
        astar(
            &(0, 0),
            |p| adjacent(*p).into_iter().filter_map(|x| {
                if x.1 >= 0 && x.0 >= 0 && x.1 < N as isize && x.0 < N as isize {
                    let c = m.get(x.1 as usize).and_then(|v| v.get(x.0 as usize))?;
                    Some((x, *c))
                } else {
                    None
                }
            }),
            |p| N as isize - p.0 + N as isize - p.1 - 2,
            |p| *p == (N as isize - 1, N as isize - 1)
        )
        .unwrap()
        .1
    );
    println!(
        "day15b: {}",
        astar(
            &(0, 0),
            |p| adjacent(*p).into_iter().filter_map(|x| {
                if x.1 >= 0 && x.0 >= 0 && x.1 < BIGN as isize && x.0 < BIGN as isize {
                    let (x, y) = x;
                    let ym = y % 100;
                    let xm = x % 100;
                    let yd = y / 100;
                    let xd = x / 100;
                    let a = m[ym as usize][xm as usize];
                    let c = ((a - 1 + yd as isize + xd as isize) % 9) + 1;
                    Some(((x, y), c))
                } else {
                    None
                }
            }),
            |p| BIGN as isize - p.0 + BIGN as isize - p.1 - 2,
            |p| *p == (BIGN as isize - 1, BIGN as isize - 1)
        )
        .unwrap()
        .1
    );
}

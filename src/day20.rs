use itertools::Itertools;
use project_root::get_project_root;

type Index = (isize, isize);
type M = (bool, usize, Vec<Vec<bool>>);
type Alg = [u64; 8];

fn calc_next(alg: &Alg, m: &M, i: Index) -> bool {
    let mut a = 0;
    for y in (i.1 - 1)..=(i.1 + 1) {
        for x in (i.0 - 1)..=(i.0 + 1) {
            let b = if x > 0 && y > 0 {
                *m.2.get((y - 1) as usize).and_then(|l| l.get((x - 1) as usize)).unwrap_or(&m.0)
            } else {
                m.0
            };
            a <<= 1;
            if b {
                a |= 1;
            }
        }
    }
    (alg[a / 64] >> (a % 64)) & 1 == 1
}
fn step(alg: &Alg, m: &mut M) {
    m.1 += 2;
    let mut output = Vec::new();
    for y in 0..m.1 {
        let mut ys = Vec::new();
        for x in 0..m.1 {
            ys.push(calc_next(alg, m, (x as isize, y as isize)));
        }
        output.push(ys);
    }
    m.0 = !m.0;
    m.2 = output;
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let (alg, mut m): (Alg, M) = {
        let (a, b) = input.split_once("\n\n").unwrap();
        let mut alg: Alg = [0; 8];
        for (n, x) in a.bytes().enumerate() {
            if x == b'#' {
                alg[n / 64] |= 1 << (n % 64);
            }
        }
        let b = b
            .lines()
            .map(|l| l.bytes().map(|c| c == b'#').collect_vec())
            .collect_vec();
        (alg, (false, b.len(), b))
    };
    for _ in 0..2 {
        step(&alg, &mut m);
    }
    println!(
        "day20a: {}",
        &m.2.iter()
            .map(|l| l.iter().filter(|x| **x).count())
            .sum::<usize>()
    );
    for _ in 0..(50 - 2) {
        step(&alg, &mut m);
    }
    println!(
        "day20b: {}",
        &m.2.iter()
            .map(|l| l.iter().filter(|x| **x).count())
            .sum::<usize>()
    );
}

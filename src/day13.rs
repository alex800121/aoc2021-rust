use aoc2021::ZipWith;
use itertools::Itertools;
use project_root::get_project_root;

const X: usize = 655 * 2 + 1;
const Y: usize = 447 * 2 + 1;
const XF: usize = 40;
const YF: usize = 6;
type Dots = [[bool; X]; Y];
fn fold_dots(dots: &mut Dots, ins: &str) {
    match ins.split_once('=') {
        Some(("fold along x", x)) => {
            let x = x.parse::<usize>().unwrap();
            for xs in dots.iter_mut() {
                for i in 0..x {
                    xs[i] = xs[i] || xs[2 * x - i];
                    xs[2 * x - i] = false;
                }
            }
        }
        Some(("fold along y", x)) => {
            let x = x.parse::<usize>().unwrap();
            for i in 0..x {
                dots[i] = dots[i].zip_with(|a, b| *a || *b, dots[2 * x - i]);
                dots[2 * x - i].iter_mut().for_each(|x| *x = false);
            }
        }
        _ => {}
    }
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let (mut dots, mut ins) = {
        let (x, y) = input.split_once("\n\n").unwrap();
        let mut dots: Dots = [[false; X]; Y];
        for (a, b) in x.lines().filter_map(|x| x.split_once(',')) {
            let a = a.parse::<usize>().unwrap();
            let b = b.parse::<usize>().unwrap();
            dots[b][a] = true;
        }
        (dots, y.lines())
    };
    // dbg!(&dots.iter().map(|xs| xs.iter().filter(|x| **x).count()).sum::<usize>());
    if let Some(ins) = ins.next() {
        fold_dots(&mut dots, ins);
    }
    println!(
        "day13a: {}",
        &dots
            .iter()
            .map(|xs| xs.iter().filter(|x| **x).count())
            .sum::<usize>()
    );
    for ins in ins {
        fold_dots(&mut dots, ins);
    }
    let s = (0..YF)
        .map(|y| {
            (0..XF)
                .map(|x| if dots[y][x] { '#' } else { ' ' })
                .collect::<String>()
        })
        .join("\n");
    println!("day13b: \n{}", s);
}

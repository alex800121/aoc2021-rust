use itertools::Itertools;
use project_root::get_project_root;

type Point = (i32, i32);
type Line = (Point, Point);

type Map = [[u16; 1000]; 1000];
fn draw_line(v: &mut Map, l: Line) {
    let x_inc = (l.1.0 - l.0.0).signum();
    let y_inc = (l.1.1 - l.0.1).signum();
    let mut p = l.0;
    loop {
        v[p.1 as usize][p.0 as usize] += 1;
        p.0 += x_inc;
        p.1 += y_inc;
        if p == l.1 {
            break;
        }
    }
    v[p.1 as usize][p.0 as usize] += 1;
}

pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input = input.lines().filter_map(|s| {
        let (a, b) = s.split_once(" -> ")?;
        let (x0, y0) = a.split_once(',')?;
        let a = (x0.parse::<i32>().ok()?, y0.parse::<i32>().ok()?);
        let (x1, y1) = b.split_once(',')?;
        let b = (x1.parse::<i32>().ok()?, y1.parse::<i32>().ok()?);
        Some((a, b))
    }).collect_vec();
    let mut v: Map = [[0; 1000]; 1000];
    input.iter().filter(|((x0, y0), (x1, y1))| x0 == x1 || y0 == y1).for_each(|&l| draw_line(&mut v, l));
    println!("day5a: {}", v.iter().map(|l| l.iter().filter(|a| **a > 1).count()).sum::<usize>());
    let mut v: Map = [[0; 1000]; 1000];
    input.iter().for_each(|&l| draw_line(&mut v, l));
    println!("day5b: {}", v.iter().map(|l| l.iter().filter(|a| **a > 1).count()).sum::<usize>());
}

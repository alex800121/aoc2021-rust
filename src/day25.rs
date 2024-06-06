use project_root::get_project_root;
use std::collections::BTreeSet;

const WIDTH: usize = 139;
const HEIGHT: usize = 137;

// const WIDTH: usize = 10;
// const HEIGHT: usize = 9;
type Index = (usize, usize);
type Floor = (BTreeSet<Index>, BTreeSet<Index>);

fn step((east, south): &Floor) -> Option<Floor> {
    let mut output = (BTreeSet::new(), BTreeSet::new());
    let mut acc = 0;
    for i @ (x, y) in east.iter() {
        let j = ((x + 1) % WIDTH, *y);
        if east.contains(&j) || south.contains(&j) {
            output.0.insert(*i);
        } else {
            output.0.insert(j);
            acc += 1;
        }
    }
    for i @ (x, y) in south.iter() {
        let j = (*x, (y + 1) % HEIGHT);
        if output.0.contains(&j) || south.contains(&j) {
            output.1.insert(*i);
        } else {
            output.1.insert(j);
            acc += 1;
        }
    }
    if acc == 0 {
        None
    } else {
        Some(output)
    }
}

fn solve(mut floor: Floor) -> u32 {
    let mut acc = 0;
    while let Some(o) = step(&floor) {
        floor = o;
        acc += 1;
    }
    acc + 1
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        // "{}/input/test{:02}.txt",
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let (mut east, mut south) = (BTreeSet::new(), BTreeSet::new());
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                'v' => {
                    south.insert((x, y));
                }
                '>' => {
                    east.insert((x, y));
                }
                _ => {}
            }
        }
    }
    println!("day25a: {}", solve((east, south)));
    println!("Merry Christmas!");
}

use itertools::Itertools;
use project_root::get_project_root;

type Submarine = (u32, u32, u32);
fn read_a(s: &str, submarine: &mut Submarine) {
    match s.split_once(' ') {
        Some(("forward", x)) => {
            submarine.0 += x.parse::<u32>().unwrap();
        },
        Some(("down", x)) => {
            submarine.1 += x.parse::<u32>().unwrap();
        },
        Some(("up", x)) => {
            submarine.1 -= x.parse::<u32>().unwrap();
        },
        _ => {},
    }
}
fn read_b(s: &str, submarine: &mut Submarine) {
    match s.split_once(' ') {
        Some(("forward", x)) => {
            let x = x.parse::<u32>().unwrap();
            submarine.0 += x;
            submarine.1 += x * submarine.2;
        },
        Some(("down", x)) => {
            submarine.2 += x.parse::<u32>().unwrap();
        },
        Some(("up", x)) => {
            submarine.2 -= x.parse::<u32>().unwrap();
        },
        _ => {},
    }
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input = input.lines().collect_vec();
    let mut init_sub = (0, 0, 0);
    for s in input.iter() {
        read_a(s, &mut init_sub);
    }
    println!("day2a: {}", init_sub.0 * init_sub.1);
    let mut init_sub = (0, 0, 0);
    for s in input.iter() {
        read_b(s, &mut init_sub);
    }
    println!("day2b: {}", init_sub.0 * init_sub.1);
}

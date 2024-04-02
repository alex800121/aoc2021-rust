use itertools::Itertools;
use project_root::get_project_root;

pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input = input
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect_vec();
    let mut n = 0;
    let mut input_a = input.iter().peekable();
    while let (Some(i0), Some(i1)) = (input_a.next(), input_a.peek()) {
        if *i1 > i0 {
            n += 1;
        }
    }
    println!("day1a: {}", n);
    let mut i = 0;
    let mut n = 0;
    while let (Some(i0), Some(i1), Some(i2), Some(i3)) = (
        input.get(i),
        input.get(i + 1),
        input.get(i + 2),
        input.get(i + 3),
    ) {
        if i0 + i1 + i2 < i1 + i2 + i3 {
            n += 1;
        }
        i += 1;
    }
    println!("day1b: {}", n);
}

use aoc2021::ZipWith;
use itertools::Itertools;
use project_root::get_project_root;

fn co2(v: &Vec<Vec<u32>>) -> u32 {
    let l0 = v.first().unwrap().len();
    let mut v = v.clone();
    let mut i = 0;
    loop {
        let l = v.len();
            // dbg!(&v);
        if l <= 1 {
            let v = v.first().unwrap();
            return v.iter().fold(0, |a, b| {
                a * 2 + b
            });
        }
        let pos_sum = v.iter().map(|x| x.get(i).unwrap()).sum::<u32>();
        if pos_sum as usize * 2 >= l {
            v.retain(|x| x.get(i).map(|a| *a == 0).unwrap());
        } else {
            v.retain(|x| x.get(i).map(|a| *a == 1).unwrap());
        }
        i += 1;
        i %= l0;
    }
}
fn oxy(v: &Vec<Vec<u32>>) -> u32 {
    let l0 = v.first().unwrap().len();
    let mut v = v.clone();
    let mut i = 0;
    loop {
        let l = v.len();
        if l <= 1 {
            let v = v.first().unwrap();
            return v.iter().fold(0, |a, b| {
                a * 2 + b
            });
        }
        let pos_sum = v.iter().filter_map(|x| x.get(i)).sum::<u32>();
        if pos_sum as usize * 2 >= l {
            v.retain(|x| x.get(i).map(|a| *a == 1).unwrap_or(false));
        } else {
            v.retain(|x| x.get(i).map(|a| *a == 0).unwrap_or(false));
        }
        i += 1;
        i %= l0;
    }
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        // "{}/input/test{:02}.txt",
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input = input
        .lines()
        .map(|x| x.chars().filter_map(|c| c.to_digit(10)).collect_vec())
        .collect_vec();
    let l0 = input.len();
    let l = input.first().unwrap().len();
    let gamma: usize = input
        .clone()
        .into_iter()
        .fold(vec![0; l], |a, b| a.zip_with(|x, y| x + y, b))
        .iter()
        .fold(0, |a, b| {
            if *b as usize * 2 > l0 {
                a * 2 + 1
            } else {
                a * 2
            }
        });
    let epsilon = 2usize.pow(l as u32) - 1 - gamma;
    println!("day3a: {}", gamma * epsilon);
    println!("day3b: {}", co2(&input) * oxy(&input));
}

use itertools::Itertools;
use project_root::get_project_root;
use std::cmp::Ordering::*;

fn bin_search<T: Fn(&Vec<i32>, i32) -> i32>(v: &Vec<i32>, f: T) -> i32 {
    let mut hi = *v.iter().max().unwrap();
    let mut lo = *v.iter().min().unwrap();
    while lo < hi {
        let mid = (hi + lo) / 2;
        match f(v, mid).cmp(&f(v, mid + 1)) {
            Less => {
                hi = mid;
            }
            _ => {
                lo = mid + 1;
            }
        }
    }
    f(v, lo)
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let v = input
        .trim()
        .split(',')
        .filter_map(|x| x.parse::<i32>().ok())
        .collect_vec();
    println!(
        "day7a: {}",
        bin_search(&v, |xs, y| xs.iter().map(|x| (x - y).abs()).sum::<i32>())
    );
    println!(
        "day7b: {}",
        bin_search(&v, |xs, y| xs
            .iter()
            .map(|x| {
                let a = (x - y).abs();
                (a * (a + 1)) / 2
            })
            .sum::<i32>())
    );
}

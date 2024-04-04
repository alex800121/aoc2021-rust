use std::array::from_fn;

use aoc2021::ZipWith;
use itertools::Itertools;
use project_root::get_project_root;

const SIG: [u8; 7] = [0, 1, 2, 3, 4, 5, 6];

fn to_id<T: Iterator<Item = u8>>(v: T) -> u8 {
    v.fold(0, |acc, x| acc + (1 << x))
}
fn decrypt(perm: &Vec<Vec<&u8>>, num: [&Vec<u8>; 10], v: [u8; 10], input: Vec<u8>) -> Vec<u8> {
    let perm = perm
        .iter()
        .find_map(|perm| from_perm(perm, num, v))
        .unwrap();
    input
        .iter()
        .filter_map(|x| {
            perm.iter()
                .find_map(|y| if y.0 == *x { Some(y.1) } else { None })
        })
        .collect_vec()
}
fn from_perm(perm: &Vec<&u8>, num: [&Vec<u8>; 10], v: [u8; 10]) -> Option<[(u8, u8); 10]> {
    let perm_num = num.map(|l| {
        to_id(l.iter().filter_map(|i| {
            let x = perm.get(*i as usize)?;
            Some(**x)
        }))
    });
    let mut perm_num = perm_num.zip_with(|a, b| (*a, *b), [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    perm_num.sort_by(|a, b| a.0.cmp(&b.0));
    if perm_num.map(|x| x.0) == v {
        Some(perm_num)
    } else {
        None
    }
}
pub fn run(day: usize) {
    let num_raw = [
        vec![0, 1, 2, 4, 5, 6],
        vec![2, 5],
        vec![0, 2, 3, 4, 6],
        vec![0, 2, 3, 5, 6],
        vec![1, 2, 3, 5],
        vec![0, 1, 3, 5, 6],
        vec![0, 1, 3, 4, 5, 6],
        vec![0, 2, 5],
        vec![0, 1, 2, 3, 4, 5, 6],
        vec![0, 1, 2, 3, 5, 6],
    ];
    let num = from_fn(|i| &num_raw[i]);
    let perm = SIG.iter().permutations(7).collect_vec();
    // dbg!(perm);
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input = input
        .lines()
        .filter_map(|x| {
            let (a, b) = x.split_once(" | ")?;
            let mut a = a
                .split_whitespace()
                .map(|x| to_id(x.bytes().map(|c| c - b'a')))
                .collect_vec();
            a.sort();
            let a = a.try_into().unwrap();
            let b = b
                .split_whitespace()
                .map(|x| to_id(x.bytes().map(|c| c - b'a')))
                .collect_vec();
            Some(decrypt(&perm, num, a, b))
        })
        .collect_vec();
    println!("day8a: {}", input.iter().map(|v| v.iter().filter(|a| [1,4,7,8].iter().any(|b| b == *a)).count()).sum::<usize>());
    println!("day8b: {}", input.iter().map(|v| v.iter().fold(0, |acc, x| acc * 10 + *x as u32)).sum::<u32>());
}

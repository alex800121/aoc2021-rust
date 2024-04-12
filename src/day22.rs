use std::{collections::HashSet, ops::Range};
use aoc2021::EucVec;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::i64,
    sequence::{preceded, separated_pair},
    IResult,
};
use project_root::get_project_root;

type V = [Range<i64>; 3];
type Ins = (bool, V);
fn read_ins(input: impl Iterator<Item = Ins>) -> HashSet<V> {
    let mut acc = HashSet::new();
    for (b, v) in input {
        acc = acc.iter().flat_map(|x: &V| x.subtract(&v)).collect::<HashSet<_>>();
        if b {
            acc.insert(v);
        } 
    }
    acc
}
fn calc_area(v: &V) -> i64 {
    v.iter().fold(1, |acc, x| acc * (x.end - x.start))
}
fn line_parser(input: &str) -> IResult<&str, Ins> {
    let (input, b) = alt((tag("on"), tag("off")))(input)?;
    let b = b == "on";
    let (input, x) = preceded(tag(" x="), separated_pair(i64, tag(".."), i64))(input)?;
    let (input, y) = preceded(tag(",y="), separated_pair(i64, tag(".."), i64))(input)?;
    let (input, z) = preceded(tag(",z="), separated_pair(i64, tag(".."), i64))(input)?;
    Ok((input, (b, [x.0..(x.1 + 1), y.0..(y.1 + 1), z.0..(z.1 + 1)])))
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input = input
        .lines()
        .filter_map(|x| line_parser(x).map(|x| x.1).ok());
    let a = read_ins(input);
    println!(
        "day22a: {}",
        a.iter()
            .map(|x| {
                x.overlap(&[-50..51, -50..51, -50..51])
                    .iter()
                    .map(calc_area)
                    .sum::<i64>()
            })
            .sum::<i64>()
    );
    println!(
        "day22b: {}",
        a.iter().map(calc_area).sum::<i64>()
    );
}

use itertools::Itertools;
use project_root::get_project_root;
use std::mem::take;

type SNum = Vec<(u32, u32)>;
fn magnitude(s: SNum) -> u32 {
    let mut left_v: Vec<(u32, u32)> = Vec::new();
    let mut tmp;
    for x in s {
        tmp = x;
        loop {
            match left_v.pop() {
                Some((x1, l1)) if tmp.1 == l1 => {
                    tmp.0 = x1 * 3 + tmp.0 * 2;
                    tmp.1 -= 1;
                }
                Some(x) => {
                    left_v.push(x);
                    left_v.push(tmp);
                    break;
                }
                None => {
                    left_v.push(tmp);
                    break;
                }
            }
        }
    }
    left_v.first().unwrap().0
}
fn reduce(s: SNum) -> SNum {
    let mut left_v: Vec<(u32, u32)> = Vec::new();
    let mut tmp: Vec<(u32, u32)> = s.clone();
    tmp.reverse();
    'a: loop {
        while let Some((x0, l0)) = tmp.pop() {
            // dbg!(&left_v, (x0, l0), &tmp);
            if l0 > 4 {
                if let Some((x1, l1)) = left_v.pop() {
                    if l0 == l1 {
                        if let Some(s) = tmp.last_mut() {
                            s.0 += x0;
                        }
                        tmp.push((0, l0 - 1));
                        if let Some(s) = left_v.pop() {
                            tmp.push((s.0 + x1, s.1));
                        }
                    } else {
                        left_v.push((x1, l1));
                        left_v.push((x0, l0));
                    }
                } else {
                    left_v.push((x0, l0));
                }
            // } else if x0 >= 10 {
            //     let a = x0 / 2;
            //     tmp.push((x0 - a, l0 + 1));
            //     tmp.push((a, l0 + 1));
            } else {
                left_v.push((x0, l0));
            }
        }
        tmp = take(&mut left_v);
        tmp.reverse();
        while let Some((x, l)) = tmp.pop() {
            if x >= 10 {
                let a = x / 2;
                tmp.push((x - a, l + 1));
                tmp.push((a, l + 1));
                continue 'a;
            } else {
                left_v.push((x, l));
            }
        }
        break;
    }
    left_v
}
fn add(a: &SNum, b: &SNum) -> SNum {
    let mut a = a.clone();
    let mut b = b.clone();
    a.iter_mut().for_each(|x| x.1 += 1);
    b.iter_mut().for_each(|x| x.1 += 1);
    a.append(&mut b);
    a
}
fn read_vec(input: &str) -> SNum {
    let mut level = 0;
    let mut output = Vec::new();
    for c in input.chars() {
        match c {
            '[' => {
                level += 1;
            }
            ']' => {
                level -= 1;
            }
            c => {
                if let Some(n) = c.to_digit(10) {
                    output.push((n, level));
                }
            }
        }
    }
    output
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        // "{}/input/test{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input = input.lines().map(read_vec).collect_vec();
    println!(
        "day18a: {}",
        input
            .clone()
            .into_iter()
            .reduce(|x, y| { reduce(add(&x, &y)) })
            .map(magnitude)
            .unwrap()
    );
    let mut n = 0;
    for x in input.iter() {
        for y in input.iter() {
            if x != y {
                n = n.max(magnitude(reduce(add(x, y))));
            }
        }
    }
    println!("day18b: {}", n);
}

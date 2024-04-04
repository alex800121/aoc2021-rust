use itertools::Itertools;
use project_root::get_project_root;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
enum SyntaxError {
    Incomplete(usize),
    Corrupted(u32),
}

const PAIRS: [(u8, u8); 4] = [(b'(', b')'), (b'[', b']'), (b'{', b'}'), (b'<', b'>')];
fn analyze(s: &str) -> SyntaxError {
    use SyntaxError::*;
    let mut v = Vec::new();
    'a: for c in s.bytes() {
        for (a, b) in PAIRS.iter() {
            if *a == c {
                v.push(c);
                continue 'a;
            }
            if c == *b {
                let x = v.pop();
                match (x, c) {
                    (Some(x), _) if *a == x => {
                        continue 'a;
                    },
                    (_, b')') => {
                        return Corrupted(3);
                    },
                    (_, b']') => {
                        return Corrupted(57);
                    },
                    (_, b'}') => {
                        return Corrupted(1197);
                    },
                    (_, b'>') => {
                        return Corrupted(25137);
                    },
                    _ => unreachable!()
                }
            }
        }
    }
    v.reverse();
    let mut acc = 0;
    'a: for c in v.iter() {
        for (i, (a, _)) in PAIRS.iter().enumerate() {
            if c == a {
                acc = acc * 5 + i + 1;
                continue 'a;
            }
        }
    }
    Incomplete(acc)
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        // "{}/input/test{:02}.txt",
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let ans = input.lines().map(analyze).collect_vec();
    println!("day10a: {}", ans.iter().filter_map(|x| match x {
        SyntaxError::Corrupted(c) => Some(*c),
        _ => None
    }).sum::<u32>());
    let mut ans = ans.iter().filter_map(|x| match x {
        SyntaxError::Incomplete(c) => Some(*c),
        _ => None
    }).collect_vec();
    ans.sort();
    println!("day10b: {}", ans.get(ans.len()/2).unwrap());
}

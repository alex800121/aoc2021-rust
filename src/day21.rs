use std::{collections::BTreeMap, mem::swap};
type Player = (u64, u64);
type State = (u64, Player, Player);
type StateB = (usize, [Player; 2]);

const INIT: State = (0, (6, 0), (2, 0));
const INIT_B: StateB = (0, [(6, 0), (2, 0)]);

fn roll(state: &mut State) -> Option<u64> {
    if state.2 .1 >= 1000 {
        return Some(state.1 .1);
    }
    for _ in 0..3 {
        state.0 %= 100;
        state.0 += 1;
        state.1 .0 += state.0 - 1;
        state.1 .0 %= 10;
        state.1 .0 += 1;
    }
    state.1 .1 += state.1 .0;
    swap(&mut state.1, &mut state.2);
    None
}
fn dirac(init: StateB) -> [u64; 2] {
    let mut dice = [[0; 10]; 4];
    dice[0][0] = 1;
    for j in 1..4 {
        for i in 0isize..10 {
            for d in ((i - 3)..=(i - 1)).filter(|x| *x >= 0) {
                dice[j][i as usize] += dice[j - 1][d as usize];
            }
        }
    }
    let dice: [(u64, u64); 10] = std::array::from_fn(|n| (n as u64, dice[3][n]));
    let mut acc = BTreeMap::from([(init, 1u64)]);
    let mut wins = [0; 2];
    while !acc.is_empty() {
        let mut new_acc = BTreeMap::new();
        'a: for (state, n) in acc.into_iter() {
            for (i, x) in state.1.iter().enumerate() {
                if x.1 >= 21 {
                    wins[i] += n;
                    continue 'a;
                }
            }
            for (m, x) in &dice[3..] {
                let mut new_state = state;
                new_state.1[new_state.0].0 += m - 1;
                new_state.1[new_state.0].0 %= 10;
                new_state.1[new_state.0].0 += 1;
                new_state.1[new_state.0].1 += new_state.1[new_state.0].0;
                new_state.0 ^= 1;
                if let Some(o) = new_acc.get_mut(&new_state) {
                    *o += n * x;
                } else {
                    new_acc.insert(new_state, n * x);
                }
            }
        }
        acc = new_acc;
    }
    wins
}
pub fn run(_day: usize) {
    let mut init_a = INIT;
    let mut acc = 0;
    let ans_a = loop {
        if let Some(n) = roll(&mut init_a) {
            break n * acc;
        }
        acc += 3;
    };
    println!("day21a: {}", ans_a);
    println!("day21b: {}", dirac(INIT_B).iter().max().unwrap());
}

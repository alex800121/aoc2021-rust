use itertools::Itertools;
use nom::AsChar;
use num::traits::Pow;
use pathfinding::prelude::dijkstra;
use project_root::get_project_root;
use std::ops::RangeInclusive;

type State<const N: usize> = [u8; N];
// 0 -> Empty
// 1 -> A
// 2 -> B
// 3 -> C
// 4 -> D
// 5 -> A complete
// 6 -> B complete
// 7 -> C complete
// 8 -> D complete

fn read_input<const N: usize>(input: &str) -> Option<State<N>> {
    let mut input = input.lines();
    let mut output = [0; N];
    let n = (N - 11) / 4;
    input.next();
    input.next()?;
    for i in 0..n {
        let cs = input.next()?.bytes().filter(|c| c.is_alpha());
        for (j, c) in cs.enumerate() {
            output[11 + i + (j * n)] = c - b'A' + 1;
        }
    }
    Some(output)
}
const ADDED: &str = "  #D#C#B#A#\n  #D#B#A#C#";
const HALL: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];
fn gen_range(a: &usize, b: &usize) -> RangeInclusive<usize> {
    *a.min(b)..=*a.max(b)
}
fn hall_to_room<const N: usize>(state: State<N>) -> impl Iterator<Item = (State<N>, usize)> {
    let levels = (N - 11) / 4;
    HALL.iter().flat_map(move |start| {
        let a = state[*start];
        let rooms = if (1..=4).contains(&a) {
            let a = a as usize - 1;
            a..(a + 1)
        } else {
            0..0
        };
        rooms.flat_map(move |room| {
            let cross = room * 2 + 2;
            let mut hall = gen_range(&cross, start);
            let base = 11 + (levels * room);
            let ls = if hall.all(|x| x == *start || state[x] == 0) {
                0..levels
            } else {
                0..0
            };
            ls.filter_map(move |level| {
                let end = base + level;
                let mut corr = base..=end;
                let mut fin = (end + 1)..(base + levels);
                let cond = state[end] == 0
                    && corr.all(|x| state[x] == 0)
                    && fin.all(|x| state[x] == a + 4);
                if cond {
                    let mut state = state;
                    state[end] = state[*start] + 4;
                    state[*start] = 0;
                    Some((
                        state,
                        (end - base + 1 + (*start as isize - cross as isize).unsigned_abs())
                            * (10.pow(a - 1) as usize),
                    ))
                } else {
                    None
                }
            })
        })
    })
}
fn room_to_hall<const N: usize>(state: State<N>) -> impl Iterator<Item = (State<N>, usize)> {
    let levels = (N - 11) / 4;
    (0..4).flat_map(move |room| {
        let base = 11 + (levels * room);
        let cross = room * 2 + 2;
        (0..levels).flat_map(move |level| {
            let start = base + level;
            let a = state[start];
            let mut corr = base..start;
            let hall = if (1..=4).contains(&a) && corr.all(|x| state[x] == 0) {
                HALL.iter()
            } else {
                [].iter()
            };
            hall.filter_map(move |end| {
                let mut hall = gen_range(&cross, end);
                let cond = hall.all(|x| state[x] == 0);
                if cond {
                    let mut state = state;
                    state[*end] = state[start];
                    state[start] = 0;
                    Some((
                        state,
                        (start - base + 1 + (*end as isize - cross as isize).unsigned_abs())
                            * (10.pow(a - 1) as usize),
                    ))
                } else {
                    None
                }
            })
        })
    })
}

pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input_a = read_input::<{ 11 + (2 * 4) }>(&input).unwrap();
    let input_b = input
        .lines()
        .take(3)
        .chain(ADDED.lines())
        .chain(input.lines().dropping(3))
        .join("\n");
    let input_b = read_input::<{ 11 + (4 * 4) }>(&input_b).unwrap();
    println!("day23a: {}", dijkstra(
        &input_a,
        |x| { room_to_hall(*x).chain(hall_to_room(*x)) },
        |x| !x.iter().any(|x| (1..=4).contains(x))
    ).unwrap().1);
    println!("day23b: {}", dijkstra(
        &input_b,
        |x| { room_to_hall(*x).chain(hall_to_room(*x)) },
        |x| !x.iter().any(|x| (1..=4).contains(x))
    ).unwrap().1);
}

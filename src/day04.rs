use itertools::Itertools;
use project_root::get_project_root;

type BingoBoard = [[(u32, bool); 5]; 5];

fn calc_board(board: BingoBoard, n: u32) -> u32 {
    board
        .iter()
        .map(|x| x.iter().map(|(a, b)| if !b { *a } else { 0 }).sum::<u32>())
        .sum::<u32>()
        * n
}
fn bingo(board: &BingoBoard) -> bool {
    for i in 0..5 {
        if board[i].iter().all(|x| x.1) || board.iter().all(|v| v[i].1) {
            return true;
        }
    }
    false
}
fn fill_n(board: &mut BingoBoard, called: u32) {
    board.iter_mut().for_each(|x| {
        x.iter_mut().for_each(|a| {
            if a.0 == called {
                a.1 = true;
            }
        })
    });
}
fn call<T: Iterator<Item = u32>>(boards: &mut Vec<BingoBoard>, called: T) -> Vec<u32> {
    let mut scores = Vec::new();
    for c in called {
        boards.iter_mut().for_each(|b| fill_n(b, c));
        let (bingoed, b): (Vec<_>, Vec<_>) = boards.iter().partition(|x| bingo(x));
        let bingoed = bingoed.iter().map(|x| calc_board(*x, c));
        scores.extend(bingoed);
        *boards = b;
    }
    scores
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let (called, mut boards) = {
        let (a, b) = input.split_once("\n\n").unwrap();
        let boards: Vec<BingoBoard> = b
            .split("\n\n")
            .map(|b| {
                b.split('\n')
                    .filter_map(|b| {
                        b.split_whitespace()
                            .filter_map(|b| b.parse::<u32>().ok().map(|b| (b, false)))
                            .collect_vec()
                            .try_into()
                            .ok()
                    })
                    .collect_vec()
                    .try_into()
                    .unwrap()
            })
            .collect_vec();
        (
            a.split(',')
                .filter_map(|x| x.parse::<u32>().ok()),
            boards,
        )
    };
    let scores = call(&mut boards, called);
    println!("day4a: {}", scores.first().unwrap());
    println!("day4a: {}", scores.last().unwrap());
}

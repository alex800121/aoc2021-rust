use itertools::Itertools;
use project_root::get_project_root;

type BingoBoard = [[(u32, bool); 5]; 5];

fn calc_board(board: BingoBoard, n: u32) -> u32 {
    unimplemented!()
}
fn bingo<T: Iterator<Item = u32>>(boards: Vec<BingoBoard>, called: T) -> Vec<u32> {
    let mut scores = Vec::new();
    for c in called {}
    scores
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let (called, boards) = {
        let (a, b) = input.split_once("\n\n").unwrap();
        let boards: Vec<BingoBoard> = b
            .split("\n\n")
            .map(|b| {
                b.split('\n')
                    .map(|b| {
                        b.split_whitespace()
                            .filter_map(|b| b.parse::<u32>().ok().map(|b| (b, false)))
                            .collect_vec()
                            .try_into()
                            .unwrap()
                    })
                    .collect_vec()
                    .try_into()
                    .unwrap()
            })
            .collect_vec();
        (a.split(", ").filter_map(|x| x.parse::<u32>().ok()), boards)
    };
    dbg!(called, boards);
}

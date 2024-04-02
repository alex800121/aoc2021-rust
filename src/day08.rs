use itertools::Itertools;
use project_root::get_project_root;

const SIG: [u8; 7] = [0, 1, 2, 3, 4, 5, 6];
pub fn run(day: usize) {
    let num = [
        vec![0, 1, 2, 4, 5, 6],
        vec![2, 5],
        vec![0, 2, 3, 4, 6],
        vec![1, 2, 3, 5],
        vec![0, 1, 3, 5, 6],
        vec![0, 1, 3, 4, 5, 6],
        vec![0, 2, 5],
        vec![0, 1, 2, 3, 4, 5, 6],
        vec![0, 1, 2, 3, 5, 6],
    ];
    let perm = SIG.iter().permutations(7);
    dbg!(num);
    // let input = std::fs::read_to_string(format!(
    //     "{}/input/input{:02}.txt",
    //     get_project_root().unwrap().to_str().unwrap(),
    //     day
    // ))
    // .unwrap();
}

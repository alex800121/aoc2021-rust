use itertools::Itertools;
use project_root::get_project_root;

fn read_block<'a, T: Iterator<Item = &'a str>>(mut input: T) -> Option<[i32; 3]> {
    let z = input.nth(4)?;
    let z = z.split_whitespace().last()?.parse::<i32>().ok()?;
    let x = input.next()?;
    let x = x.split_whitespace().last()?.parse::<i32>().ok()?;
    let y = input.nth(9)?;
    let y = y.split_whitespace().last()?.parse::<i32>().ok()?;
    Some([z, x, y])
}
type Zxy = [[i32; 3]; 14];
fn solve(input: &mut [i32; 14], zxy: &Zxy) {
    let mut stack: Vec<(usize, i32)> = Vec::new();
    for (i, [z, x, y]) in zxy.iter().enumerate() {
        match &z {
            1 => {
                stack.push((i, *y));
            },
            26 => {
                if let Some((j, y)) = stack.pop() {
                    // input[j] + y + x == input[i]
                    input[i] = input[j] + y + x;
                    if input[i] > 9 {
                        input[j] -= input[i] - 9;
                        input[i] = 9;
                    }
                    if input [i] < 1 {
                        input[j] += 1 - input[i];
                        input[i] = 1;
                    }
                }
            },
            _ => {},
        }
    }
    assert_eq!(stack.len(), 0);
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input: Zxy = input
        .lines()
        .chunks(252 / 14)
        .into_iter()
        .filter_map(read_block)
        .collect_vec()
        .try_into()
        .unwrap();
    assert_eq!(input.iter().filter(|&&x| x[0] == 1 && x[1] > 9).count(), 7);
    assert_eq!(input.iter().filter(|&&x| x[0] == 26).count(), 7);
    let mut input_a = [9; 14];
    solve(&mut input_a, &input);
    println!("day24a: {}", input_a.map(|x| x.to_string()).concat());
    let mut input_b = [1; 14];
    solve(&mut input_b, &input);
    println!("day24b: {}", input_b.map(|x| x.to_string()).concat());
}

use itertools::Itertools;
use nom::AsChar;
use project_root::get_project_root;

type State<const N: usize> = [u8; N];

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
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input_a = read_input::<{ 11 + (2 * 4) }>(&input).unwrap();
    dbg!(input_a);
    let input_b = input
        .lines()
        .take(3)
        .chain(ADDED.lines())
        .chain(input.lines().dropping(3))
        .join("\n");
    let input_b = read_input::<{ 11 + (4 * 4) }>(&input_b).unwrap();
    dbg!(input_b);
}

use project_root::get_project_root;

type Fish = [u64; 9];

fn reproduce(fish: Fish, days: usize) -> Fish {
    let mut fish = fish;
    for _ in 0..days {
        let n = fish[0];
        fish = std::array::from_fn(|i| *fish.get(i + 1).unwrap_or(&0));
        fish[8] += n;
        fish [6] += n;
    }
    fish
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let mut fish: Fish = [0; 9];
    input.trim().split(',').for_each(|x| {
        let i = x.parse::<usize>().unwrap();
        fish[i] += 1;
    });
    println!("day6a: {}", reproduce(fish, 80).iter().sum::<u64>());
    println!("day6b: {}", reproduce(fish, 256).iter().sum::<u64>());
}

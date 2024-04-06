use itertools::Either;
use project_root::get_project_root;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Packet {
    version: u8,
    type_id: u8,
    sub_packets: Either<u64, Vec<Packet>>,
}
type Bin = Vec<bool>;

fn to_num(input: &[bool]) -> u64 {
    let mut acc = 0;
    for i in input {
        acc *= 2;
        acc += if *i { 1 } else { 0 };
    }
    acc
}
fn parse_packet(input: &[bool]) -> (&[bool], Packet) {
    dbg!(input.len());
    let version = to_num(&input[0..3]) as u8;
    let type_id = to_num(&input[3..6]) as u8;
    if 4 == type_id {
        let mut index = 6;
        let mut acc = 0;
        while let Some(n) = input.get(index) {
            acc <<= 4;
            acc += to_num(&input[(index + 1)..(index + 5)]);
            index += 5;
            if !n {
                return (
                    &input[index..],
                    Packet {
                        version,
                        type_id,
                        sub_packets: Either::Left(acc),
                    },
                );
            }
        }
    }
    if let Some(false) = input.get(6) {
        let total_len = to_num(&input[7..22]) as usize;
        let mut sub_packets = Vec::new();
        let next_input = loop {
            let (next_input, p) = parse_packet(&input[22..(22 + total_len)]);
            sub_packets.push(p);
            if next_input.is_empty() {
                break next_input;
            }
        };
        (
            next_input,
            Packet {
                version,
                type_id,
                sub_packets: Either::Right(sub_packets),
            },
        )
    } else {
        let packet_len = to_num(&input[7..18]);
        let mut sub_packets = Vec::new();
        let mut next_input = &input[18..];
        for _ in 0..packet_len {
            let (p, v) = parse_packet(next_input);
            sub_packets.push(v);
            next_input = p;
        }
        (
            next_input,
            Packet {
                version,
                type_id,
                sub_packets: Either::Right(sub_packets),
            },
        )
    }
}

fn to_bin(input: &str) -> Option<Bin> {
    let mut output = Vec::new();
    for c in input.chars() {
        let s = format!("{:04b}", c.to_digit(16)?);
        // dbg!(&s);
        for c in s.chars() {
            output.push(c == '1');
        }
    }
    Some(output)
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input = to_bin(input.trim()).unwrap();
    let (input, ans) = parse_packet(&input[..]);
    dbg!(ans);
}

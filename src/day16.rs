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
    // dbg!(input.len());
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
        let mut m = &input[22..(22 + total_len)];
        loop {
            let (n, p) = parse_packet(m);
            sub_packets.push(p);
            if n.is_empty() {
                break;
            }
            m = n;
        }
        (
            &input[(22 + total_len)..],
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
fn calc_version(p: &Packet) -> u64 {
    match p {
        Packet {
            version: v,
            type_id: _,
            sub_packets: Either::Left(_),
        } => *v as u64,
        Packet {
            version: v,
            type_id: _,
            sub_packets: Either::Right(ps),
        } => *v as u64 + (ps.iter().map(calc_version).sum::<u64>()),
    }
}
fn calc(p: &Packet) -> u64 {
    match p {
        Packet {
            version: _,
            type_id: 0,
            sub_packets: Either::Right(s),
        } => s.iter().map(calc).sum::<u64>(),
        Packet {
            version: _,
            type_id: 1,
            sub_packets: Either::Right(s),
        } => s.iter().map(calc).product::<u64>(),
        Packet {
            version: _,
            type_id: 2,
            sub_packets: Either::Right(s),
        } => s.iter().map(calc).min().unwrap(),
        Packet {
            version: _,
            type_id: 3,
            sub_packets: Either::Right(s),
        } => s.iter().map(calc).max().unwrap(),
        Packet {
            version: _,
            type_id: 4,
            sub_packets: Either::Left(s),
        } => *s,
        Packet {
            version: _,
            type_id: 5,
            sub_packets: Either::Right(s),
        } => {
            if calc(s.first().unwrap()) > calc(s.get(1).unwrap()) {
                1
            } else {
                0
            }
        },
        Packet {
            version: _,
            type_id: 6,
            sub_packets: Either::Right(s),
        } => {
            if calc(s.first().unwrap()) < calc(s.get(1).unwrap()) {
                1
            } else {
                0
            }
        },
        Packet {
            version: _,
            type_id: 7,
            sub_packets: Either::Right(s),
        } => {
            if calc(s.first().unwrap()) == calc(s.get(1).unwrap()) {
                1
            } else {
                0
            }
        },
        _ => unreachable!(),
    }
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        // "{}/input/test{:02}.txt",
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input = to_bin(input.trim()).unwrap();
    let (_input, ans) = parse_packet(&input[..]);
    println!("day16a: {}", calc_version(&ans));
    println!("day16b: {}", calc(&ans));
}

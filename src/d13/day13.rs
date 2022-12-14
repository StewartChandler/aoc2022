use std::{
    cmp::Ordering,
    error::Error,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    List(Vec<Packet>),
    Num(i64),
}

#[derive(Debug)]
struct PacketParseError(String);

impl Display for PacketParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for PacketParseError {}

impl FromStr for Packet {
    type Err = PacketParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix('[')
            .ok_or(PacketParseError(format!(
                "String `{s}` does not begin with ["
            )))?
            .strip_suffix(']')
            .ok_or(PacketParseError(format!(
                "String `{s}` does not end with ]"
            )))?
            .trim();

        if s.is_empty() {
            return Ok(Packet::List(vec![]));
        }

        let mut list_stack: Vec<Vec<Packet>> = vec![vec![]];
        for mut ss in s.split(',') {
            loop {
                ss = match ss.strip_prefix('[') {
                    Some(ss) => {
                        list_stack.push(vec![]);
                        ss
                    }
                    None => {
                        break;
                    }
                };
            }

            match ss.split_once(']').map(|(ss, b)| (ss.trim(), b)) {
                Some((ss, b)) => {
                    let mut vec = list_stack
                        .pop()
                        .ok_or(PacketParseError("Unmatched `]`".into()))?;
                    if !ss.is_empty() {
                        vec.push(Packet::Num(
                            ss.parse::<i64>()
                                .map_err(|err| PacketParseError(err.to_string()))?,
                        ));
                    }
                    list_stack
                        .last_mut()
                        .ok_or(PacketParseError("Missing `]`".into()))?
                        .push(Packet::List(vec));

                    for c in b.chars() {
                        if c == ']' {
                            let vec = list_stack
                                .pop()
                                .ok_or(PacketParseError("Unmatched `]`".into()))?;
                            list_stack
                                .last_mut()
                                .ok_or(PacketParseError("Missing `]`".into()))?
                                .push(Packet::List(vec));
                        } else if !c.is_whitespace() {
                            return Err(PacketParseError(format!("Unexpected token `{c}`")));
                        }
                    }
                }
                None => {
                    list_stack
                        .last_mut()
                        .ok_or(PacketParseError("Missing `]`".into()))?
                        .push(Packet::Num(
                            ss.trim()
                                .parse::<i64>()
                                .map_err(|err| PacketParseError(err.to_string()))?,
                        ));
                }
            }
        }

        if list_stack.len() != 1 {
            Err(PacketParseError("Missing `]`".into()))
        } else {
            Ok(Packet::List(list_stack.pop().unwrap()))
        }
    }
}

fn compare_packets(a: &Packet, b: &Packet) -> Ordering {
    match (a, b) {
        (Packet::Num(a), Packet::Num(b)) => a.cmp(b),
        (Packet::List(a), Packet::Num(b)) => a.cmp(&vec![Packet::Num(*b)]),
        (Packet::Num(a), Packet::List(b)) => vec![Packet::Num(*a)].cmp(b),
        (Packet::List(a), Packet::List(b)) => a.cmp(b),
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(compare_packets(self, other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        compare_packets(self, other)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let fpath = std::env::args().nth(1).expect("no file argument found");

    let mut lines = BufReader::new(File::open(fpath)?).lines().flatten();

    let pairs: Vec<_> = std::iter::from_fn(|| {
        lines
            .next()
            .map(|s1| s1.parse::<Packet>().unwrap())
            .and_then(|p1| lines.next().map(|s2| (p1, s2)))
            .map(|(p1, s2)| (p1, s2.parse::<Packet>().unwrap()))
            .map(|x| {
                lines.next();
                x
            })
    })
    .collect();

    // part 1
    println!(
        "Part 1: {}",
        pairs
            .iter()
            .enumerate()
            .filter(|(_, (l, r))| l < r)
            .map(|(i, _)| i + 1)
            .sum::<usize>()
    );

    // part 2
    let d1 = Packet::List(vec![Packet::List(vec![Packet::Num(2)])]);
    let d2 = Packet::List(vec![Packet::List(vec![Packet::Num(6)])]);

    let (i1, i2) = pairs
        .into_iter()
        .flat_map(|(l, r)| [l, r].into_iter())
        .fold((1u32, 2u32), |(i1, i2), p| {
            if p < d1 {
                (i1 + 1, i2 + 1)
            } else if p < d2 {
                (i1, i2 + 1)
            } else {
                (i1, i2)
            }
        });
    println!("Part 2: {}", i1 * i2);

    Ok(())
}

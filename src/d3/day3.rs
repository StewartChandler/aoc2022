use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn priority(c: &u8) -> u8 {
    match *c {
        c if (b'A'..=b'Z').contains(&c) => c - b'A' + 27,
        c if (b'a'..=b'z').contains(&c) => c - b'a' + 1,
        _ => panic!("invalid rucksack item"),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let fpath = std::env::args().nth(1).expect("no file argument found");

    let mut rucksacks: Vec<(u64, u64)> = Vec::new();
    for s in BufReader::new(File::open(fpath)?).lines().flatten() {
        let b = s.as_bytes();
        let (l, r) = b.split_at(b.len() / 2);
        rucksacks.push(l.iter().zip(r).fold((0u64, 0u64), |(la, ra), (l, r)| {
            (la | 1u64 << priority(l), ra | 1u64 << priority(r))
        }));
    }

    // eprintln!("{:?}", rucksacks);

    // part 1
    println!(
        "Part 1: {}",
        rucksacks
            .iter()
            .map(|(l, r)| (l & r).trailing_zeros())
            .sum::<u32>()
    );

    // part 2
    println!(
        "Part 2: {}",
        rucksacks
            .chunks_exact(3)
            .map(
                |lr| ((lr[0].0 | lr[0].1) & (lr[1].0 | lr[1].1) & (lr[2].0 | lr[2].1))
                    .trailing_zeros()
            )
            .sum::<u32>()
    );

    Ok(())
}

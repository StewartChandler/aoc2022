use std::{error::Error, fs::File, io::{BufRead, BufReader}};

fn priority(c: &u8) -> u8 {
    match *c {
        c if c <= b'Z' && c >= b'A' => c - b'A' + 27,
        c if c <= b'z' && c >= b'a' => c - b'a' + 1,
        _ => panic!("invalid rucksack item"),
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let fpath = std::env::args().nth(1).expect("no file argument found");

    let mut rucksacks: Vec<(u64, u64)> = Vec::new();
    for line in BufReader::new(File::open(fpath)?).lines() {
        if let Ok(s) = line {
            let b = s.as_bytes();
            let (l, r) = b.split_at(b.len()/2);
            rucksacks.push(l
                .iter()
                .zip(r)
                .fold((0u64, 0u64), |(la, ra), (l, r)| {
                    (la | 1u64 << priority(l), ra | 1u64 << priority(r))
                })
            );
        }
    }

    // eprintln!("{:?}", rucksacks);

    // part 1
    println!("Part 1: {}", rucksacks.iter()
        .map(|(l, r)| (l & r).trailing_zeros())
        .sum::<u32>()
    );

    // part 2
    let mut sum = 0u32;
    let mut itr = rucksacks.iter().map(|(l, r)| (l | r));
    while let Some(val) = itr.next() {
        sum += (
            val & 
            itr.next().unwrap_or(0) & 
            itr.next().unwrap_or(0)
        ).trailing_zeros();
    }
    println!("Part 2: {}", sum);

    Ok(())
}
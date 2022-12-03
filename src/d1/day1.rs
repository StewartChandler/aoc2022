use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut cals: Vec<Vec<u32>> = vec![];
    let fpath = std::env::args().nth(1).expect("no file argument found");

    let input = File::open(fpath)?;
    let reader = BufReader::new(input);

    for line in reader.lines() {
        match line?.parse::<u32>() {
            Ok(x) => {
                if let Some(cal) = cals.last_mut() {
                    cal.push(x);
                } else {
                    cals.push(vec![x]);
                }
            }
            Err(_) => {
                // not a number, use as seperator
                cals.push(vec![]);
            }
        }
    }

    if let Some(m) = cals.iter().map(|v| v.iter().sum::<u32>()).max() {
        println!("Part 1: {}", m);
    } else {
        eprintln!("no max could be found");
    }

    let mut totals: Vec<_> = cals
        .into_iter()
        .map(|v| v.into_iter().sum::<u32>())
        .collect();
    totals.sort();
    println!("Part 2: {}", totals.into_iter().rev().take(3).sum::<u32>());

    Ok(())
}

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
    let i = totals.len() - 3;
    totals.select_nth_unstable(i);
    println!(
        "Part 2: {}",
        totals[totals.len() - 3..].into_iter().sum::<u32>()
    );

    Ok(())
}

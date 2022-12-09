use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let fpath = std::env::args().nth(1).expect("no file argument found");

    let mut pairs: Vec<[u32; 4]> = Vec::new();
    for s in BufReader::new(File::open(fpath)?).lines().flatten() {
        let nums: Vec<u32> = s
            .split(&[',', '-'])
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        pairs.push(nums.try_into().unwrap());
    }

    // eprintln!("{:#?}", pairs);

    // part 1
    println!(
        "Part 1: {}",
        pairs.iter().fold(0, |sum, arr| {
            if (arr[0] >= arr[2] && arr[1] <= arr[3]) || (arr[0] <= arr[2] && arr[1] >= arr[3]) {
                sum + 1
            } else {
                sum
            }
        })
    );

    // part 2
    println!(
        "Part 2: {}",
        pairs.iter().fold(0, |sum, arr| {
            if arr[0] > arr[3] || arr[1] < arr[2] {
                sum
            } else {
                sum + 1
            }
        })
    );

    Ok(())
}

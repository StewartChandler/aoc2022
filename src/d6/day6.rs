use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
    iter::successors,
};

fn solve_n_distinct<const N: usize>(data: &String) -> usize {
    let b = data.as_bytes();
    successors(Some((0, N - 1)), move |&(i, by)| {
        (i + N - 1 - by..i + N - 1)
            .rev()
            .find(|&j| (j + 1..i + N).rev().any(|k| b[k] == b[j]))
            .and_then(|x| Some((x + 1, x + 1 - i)))
            .or_else(|| {
                (i..i + N - 1 - by)
                    .rev()
                    .find(|&j| (i + N - 1 - by..i + N).rev().any(|k| b[k] == b[j]))
                    .and_then(|x| Some((x + 1, x + 1 - i)))
            })
    })
    .last()
    .unwrap()
    .0 + N
}

fn main() -> Result<(), Box<dyn Error>> {
    let fpath = std::env::args().nth(1).expect("no file argument found");

    let mut data = String::new();
    BufReader::new(File::open(fpath)?).read_to_string(&mut data)?;

    // eprintln!("{:?}", data);

    // part 1
    println!("Part 1: {}", solve_n_distinct::<4>(&data));

    // part 2
    println!("Part 2: {}", solve_n_distinct::<14>(&data));

    Ok(())
}

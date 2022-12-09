use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let fpath = std::env::args().nth(1).expect("no file argument found");

    let mut l_iter = BufReader::new(File::open(fpath)?)
        .lines()
        .filter_map(|r| r.ok())
        .filter(|s| !s.is_empty())
        .peekable();

    let stride = l_iter.peek().unwrap().as_bytes().len();
    let trees: Vec<u8> = l_iter
        .flat_map(|s| s.into_bytes().into_iter())
        .map(|b| b - b'0')
        .collect();

    let rows = trees.len() / stride;

    let visible: usize = trees
        .iter()
        .enumerate()
        .filter(|&(i, &h)| {
            let row = i / stride;
            let col = i % stride;
            (stride * row..i).all(|o| trees[o] < h)
                || (i + 1..stride * (row + 1)).all(|o| trees[o] < h)
                || (col..i).step_by(stride).all(|o| trees[o] < h)
                || (i + stride..trees.len() + col)
                    .step_by(stride)
                    .all(|o| trees[o] < h)
        })
        .count();
    println!("Part 1: {visible}");

    let max_scenic = trees
        .iter()
        .enumerate()
        .map(|(i, &h)| {
            let row = i / stride;
            let col = i % stride;
            (stride * row..i)
                .rev()
                .position(|o| trees[o] >= h)
                .map(|v| v + 1)
                .unwrap_or(i - stride * row)
                * (i + 1..stride * (row + 1))
                    .position(|o| trees[o] >= h)
                    .map(|v| v + 1)
                    .unwrap_or(stride * (row + 1) - (i + 1))
                * (col..i)
                    .step_by(stride)
                    .rev()
                    .position(|o| trees[o] >= h)
                    .map(|v| v + 1)
                    .unwrap_or(row)
                * (i + stride..trees.len() + col)
                    .step_by(stride)
                    .position(|o| trees[o] >= h)
                    .map(|v| v + 1)
                    .unwrap_or(rows - row - 1)
        })
        .max()
        .unwrap();
    println!("Part 2: {max_scenic}");

    Ok(())
}

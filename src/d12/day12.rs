use std::{
    collections::VecDeque,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn bfs(adj: &Vec<Vec<usize>>, start: usize, end: usize) -> Option<u64> {
    let mut explored = vec![false; adj.len()];
    explored[start] = true;
    let mut q: VecDeque<(usize, u64)> = VecDeque::new();
    q.push_back((start, 0));
    while let Some((i, moves)) = q.pop_front() {
        // eprintln!("pos: ({}, {}) moves: {moves:>3}", i % width, i / width);
        if i == end {
            return Some(moves);
        } else {
            q.extend(
                adj[i]
                    .iter()
                    .copied()
                    .filter(|&j| {
                        if !explored[j] {
                            explored[j] = true;
                            true
                        } else {
                            false
                        }
                    })
                    .zip(std::iter::repeat(moves + 1)),
            );
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let fpath = std::env::args().nth(1).expect("no file argument found");

    let mut lines = BufReader::new(File::open(fpath)?)
        .lines()
        .flatten()
        .peekable();

    let width = lines.peek().unwrap().len();

    let mut map: Vec<u8> = lines
        .filter(|s| !s.is_empty())
        .flat_map(|s| s.into_bytes())
        .collect();

    let height = map.len() / width;

    let start = map.iter().position(|b| *b == b'S').unwrap();
    let end = map.iter().position(|b| *b == b'E').unwrap();

    map[end] = b'z'; // end has elevation 'z'

    let adj: Vec<Vec<usize>> = (0..map.len())
        .map(|i| {
            let x = i % width;
            let y = i / width;

            let mut a = vec![];
            if x != 0 && (map[i - 1] <= map[i] + 1 || map[i] == b'S') {
                a.push(i - 1);
            }
            if x < width - 1 && (map[i + 1] <= map[i] + 1 || map[i] == b'S') {
                a.push(i + 1);
            }
            if y < height - 1 && (map[i + width] <= map[i] + 1 || map[i] == b'S') {
                a.push(i + width);
            }
            if y != 0 && (map[i - width] <= map[i] + 1 || map[i] == b'S') {
                a.push(i - width);
            }

            a
        })
        .collect();

    // part 1
    println!("Part 1 {}", bfs(&adj, start, end).unwrap());

    // part 2
    // there are much faster ways to do this,
    // might rewrite, but I got an exam tomorrow and this works.
    println!(
        "Part 2: {}",
        map.iter()
            .enumerate()
            .filter_map(|(i, &b)| (b == b'a').then_some(i))
            .flat_map(|i| bfs(&adj, i, end))
            .min()
            .unwrap()
    );

    Ok(())
}

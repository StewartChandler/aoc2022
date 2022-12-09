use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let fpath = std::env::args().nth(1).expect("no file argument found");

    let lines: Vec<String> = BufReader::new(File::open(fpath)?)
        .lines()
        .into_iter()
        .filter_map(|r| r.ok())
        .collect();

    // part 1
    let mut size_stack: Vec<u64> = Vec::new();
    let result: u64 = lines
        .iter()
        .filter_map(|s| {
            let mut token = s.split_whitespace();
            match token.next().unwrap() {
                "$" => match token.next().unwrap() {
                    "cd" => match token.next().unwrap() {
                        ".." => {
                            // pop size
                            let size = size_stack.pop().unwrap();
                            let stacklen = size_stack.len();
                            if stacklen > 0 {
                                size_stack[stacklen - 1] += size;
                            }
                            Some(size)
                        }
                        _ => {
                            size_stack.push(0);
                            None
                        }
                    },
                    "ls" => None,
                    _ => panic!("unknown command!"),
                },
                s => s.parse::<u64>().ok().and_then(|x| {
                    let stacklen = size_stack.len();
                    if stacklen > 0 {
                        size_stack[stacklen - 1] += x;
                    }
                    None
                }),
            }
        })
        .filter(|x| *x <= 100000)
        .sum::<u64>()
        + {
            let mut iter = size_stack // unwind stack
                .into_iter()
                .rev();
            std::iter::successors(Some(0u64), move |x| iter.next().and_then(|y| Some(x + y)))
                .skip(1)
                .take_while(|x| *x <= 100000)
                .sum::<u64>()
        };

    println!("Part 1 {}", result);

    // part 2
    let space_needed = lines
            .iter()
            .filter_map(|s| {
                s.split_whitespace()
                    .next()
                    .and_then(|t| t.parse::<u64>().ok())
            })
            .sum::<u64>() - 40000000;

    let mut size_stack: Vec<u64> = Vec::new();
    let result: u64 = match (
        lines
            .iter()
            .filter_map(|s| {
                let mut token = s.split_whitespace();
                match token.next().unwrap() {
                    "$" => match token.next().unwrap() {
                        "cd" => match token.next().unwrap() {
                            ".." => {
                                // pop size
                                let size = size_stack.pop().unwrap();
                                let stacklen = size_stack.len();
                                if stacklen > 0 {
                                    size_stack[stacklen - 1] += size;
                                }
                                Some(size)
                            }
                            _ => {
                                size_stack.push(0);
                                None
                            }
                        },
                        "ls" => None,
                        _ => panic!("unknown command!"),
                    },
                    s => s.parse::<u64>().ok().and_then(|x| {
                        let stacklen = size_stack.len();
                        if stacklen > 0 {
                            size_stack[stacklen - 1] += x;
                        }
                        None
                    }),
                }
            })
            .filter(|x| *x >= space_needed)
            .min(),
        {
            let mut iter = size_stack // unwind stack
                .into_iter()
                .rev();
            std::iter::successors(Some(0u64), move |x| iter.next().and_then(|y| Some(x + y)))
                .skip(1)
                .find(|x| *x >= space_needed)
        },
    ) {
        (Some(x), Some(y)) => x.min(y),
        (Some(x), None) => x,
        (None, Some(y)) => y,
        (None, None) => panic!("No minimum"),
    };
    println!("Part 2: {}", result);

    Ok(())
}

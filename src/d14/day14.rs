use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let fpath = std::env::args().nth(1).expect("no file argument found");

    let paths: Vec<Vec<(_, _)>> = BufReader::new(File::open(fpath)?)
        .lines()
        .flatten()
        .map(|s| {
            s.split("->")
                .map(|ss| {
                    ss.split_once(',')
                        .map(|(s1, s2)| {
                            (
                                s1.trim().parse::<usize>().unwrap(),
                                s2.trim().parse::<usize>().unwrap(),
                            )
                        })
                        .unwrap()
                })
                .collect()
        })
        .collect();

    let y_max = paths.iter().flatten().map(|(_, y)| *y).max().unwrap();

    let x_min = std::cmp::min(
        paths.iter().flatten().map(|(x, _)| *x).min().unwrap(),
        500 - 2 - y_max,
    );
    let x_max = std::cmp::max(
        paths.iter().flatten().map(|(x, _)| *x).max().unwrap(),
        500 + 2 + y_max,
    );

    let height = y_max + 1;
    let width = x_max - x_min + 1;

    let mut cave = vec![b'.'; width * height];

    paths.iter().for_each(|p| {
        (0..p.len() - 1)
            .map(|i| (p[i].0 - x_min, p[i].1, p[i + 1].0 - x_min, p[i + 1].1))
            .for_each(|(mut x1, mut y1, mut x2, mut y2)| {
                if x1 == x2 {
                    if y1 > y2 {
                        std::mem::swap(&mut y1, &mut y2);
                    }
                    (x1 + y1 * width..=x1 + y2 * width).step_by(width)
                } else {
                    if x1 > x2 {
                        std::mem::swap(&mut x1, &mut x2);
                    }
                    (x1 + y1 * width..=x2 + y1 * width).step_by(1)
                }
                .for_each(|i| cave[i] = b'#');
            });
    });

    // part 1
    let mut result = std::iter::from_fn(|| {
        let mut pos = 500 - x_min; // (500, 0)

        loop {
            if pos + width < cave.len() && cave[pos + width] == b'.' {
                pos += width;
            } else if pos + width - 1 < cave.len() && cave[pos + width - 1] == b'.' {
                pos += width - 1;
            } else if pos + width + 1 < cave.len() && cave[pos + width + 1] == b'.' {
                pos += width + 1;
            } else {
                break;
            }
        }

        if pos < cave.len() - width {
            cave[pos] = b'o';
            Some(())
        } else {
            None
        }
    })
    .count();

    // cave.chunks(width).map(|arr| std::str::from_utf8(arr).unwrap()).for_each(|l| {
    //     eprintln!("{l}");
    // });

    println!("Part 1: {result}");

    // part 2
    let new_width = x_max - x_min + 1;

    cave.append(&mut vec![b'.'; new_width]);

    result += std::iter::from_fn(|| {
        let mut pos = 500 - x_min; // (500, 0)

        if cave[pos] != b'.' {
            return None;
        }

        loop {
            if pos + width < cave.len() && cave[pos + width] == b'.' {
                pos += width;
            } else if pos + width - 1 < cave.len() && cave[pos + width - 1] == b'.' {
                pos += width - 1;
            } else if pos + width + 1 < cave.len() && cave[pos + width + 1] == b'.' {
                pos += width + 1;
            } else {
                break;
            }
        }

        if pos < cave.len() {
            cave[pos] = b'o';
            Some(())
        } else {
            None
        }
    })
    .count();

    // cave.chunks(width).map(|arr| std::str::from_utf8(arr).unwrap()).for_each(|l| {
    //     eprintln!("{l}");
    // });
    // eprintln!("{}", std::str::from_utf8(&vec![b'#'; new_width]).unwrap());

    println!("Part 2: {result}");

    Ok(())
}

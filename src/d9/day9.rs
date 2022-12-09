use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader}, 
    collections::HashSet,
};

fn main() -> Result<(), Box<dyn Error>> {
    let fpath = std::env::args().nth(1).expect("no file argument found");

    let lines: Vec<_> = BufReader::new(File::open(fpath)?)
        .lines()
        .filter_map(|r| r.ok())
        .filter(|s| !s.is_empty()).collect();

    // part 1
    let pos: HashSet<_> = lines
        .iter()
        .flat_map(|s| std::iter::repeat(s.as_bytes()[0]).take(s[2..].parse::<usize>().unwrap()))
        .scan(((0, 0), (0, 0)), |((tx, ty), (hx, hy)), b| {
            match b {
                b'U' => { 
                    if *ty < *hy {
                        *ty = *hy;
                        *tx = *hx;
                    } 
                    *hy += 1;
                },
                b'R' => {
                    if *tx < *hx {
                        *ty = *hy;
                        *tx = *hx;
                    } 
                    *hx += 1;
                }
                b'D' => {
                    if *ty > *hy {
                        *ty = *hy;
                        *tx = *hx;
                    } 
                    *hy -= 1;
                }
                b'L' => {
                    if *tx > *hx {
                        *ty = *hy;
                        *tx = *hx;
                    } 
                    *hx -= 1;
                }
                b => panic!("Unknown input: {}", b),
            };

            Some((*tx, *ty))
        }).collect();

    println!("Part 1: {}", pos.len());

    // part 2
    let pos: HashSet<_> = lines
        .iter()
        .flat_map(|s| std::iter::repeat(s.as_bytes()[0]).take(s[2..].parse::<usize>().unwrap()))
        .scan([(0i32, 0i32); 10], |arr, b| {
            match b {
                b'U' => { 
                    arr[0].1 += 1;
                },
                b'R' => {
                    arr[0].0 += 1;
                }
                b'D' => {
                    arr[0].1 -= 1;
                }
                b'L' => {
                    arr[0].0 -= 1;
                }
                b => panic!("Unknown input: {}", b),
            };

            for i in 1..10 {
                let (px, py) = arr[i - 1];
                let (cx, cy) = &mut arr[i];

                let dx = px - *cx;
                let dy = py - *cy;
                if dx > 1 || dx < -1 || dy > 1 || dy < -1{
                    *cx += dx.signum();
                    *cy += dy.signum();
                } else {
                    break;
                }
            }

            Some(arr[9])
        }).collect();

    println!("Part 2: {}", pos.len());


    Ok(())
}

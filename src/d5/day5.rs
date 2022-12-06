use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[inline]
fn get_pair_mut<T>(v: &mut Vec<T>, first: usize, second: usize) -> Result<(&mut T, &mut T), ()> {
    if first == second {
        Err(())
    } else {
        // Safety:
        //   So long as `first` and `second` do not alias, which we check,
        //   the mutable references `&mut v[first]` and `&mut v[second]` must not alias.
        assert!(first < v.len());
        assert!(second < v.len());
        Ok(unsafe {
            (
                v.as_mut_ptr().add(first).as_mut().unwrap_unchecked(),
                v.as_mut_ptr().add(second).as_mut().unwrap_unchecked(),
            )
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let fpath = std::env::args().nth(1).expect("no file argument found");

    let mut s_lines: Vec<String> = Vec::new();
    let mut lines = BufReader::new(File::open(fpath)?).lines();
    while let Some(Ok(s)) = lines.next() {
        if s.is_empty() {
            break;
        }

        s_lines.push(s);
    }

    let len = s_lines.pop().unwrap().len() / 4 + 1;

    let mut stacks: Vec<Vec<u8>> = (0..len).map(|_| vec![]).collect();

    s_lines
        .into_iter()
        .rev()
        .map(|s| s.into_bytes())
        .for_each(|b| {
            b.chunks(4)
                .enumerate()
                .filter(|(_, x)| x[0] == b'[')
                .for_each(|(i, b)| stacks[i].push(b[1]))
        });

    // eprintln!("{:?}", stacks);

    let moves: Vec<_> = lines
        .into_iter()
        .map(|r| r.unwrap())
        .filter_map(|s| {
            s.strip_prefix("move ").and_then(|ss| {
                ss.split_once(" from ").and_then(|(s1, s2)| {
                    let n1 = s1.parse::<usize>().ok()?;
                    let (n2, n3) = s2.split_once(" to ").and_then(|(s3, s4)| {
                        Some((s3.parse::<usize>().ok()?, s4.parse::<usize>().ok()?))
                    })?;
                    Some((n1, n2, n3))
                })
            })
        })
        .collect();

    // eprintln!("{:?}", moves);

    // part 1
    let mut stacks1 = stacks.clone();
    moves.iter().for_each(|(m, f, t)| {
        if let Ok((from, to)) = get_pair_mut(&mut stacks1, *f - 1, *t - 1) {
            to.extend(from.drain(from.len() - m..).rev());
        }
    });

    println!(
        "Part 1: {}",
        String::from_utf8(
            stacks1
                .into_iter()
                .map(|stack| stack[stack.len() - 1])
                .collect()
        )
        .unwrap()
    );

    // part 2
    moves.iter().for_each(|(m, f, t)| {
        if let Ok((from, to)) = get_pair_mut(&mut stacks, *f - 1, *t - 1) {
            to.extend(from.drain(from.len() - m..));
        }
    });

    println!(
        "Part 2: {}",
        String::from_utf8(
            stacks
                .into_iter()
                .map(|stack| stack[stack.len() - 1])
                .collect()
        )
        .unwrap()
    );

    Ok(())
}

use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
    str::FromStr,
};

#[derive(Debug, Clone, Copy)]
enum Var {
    Literal(i64),
    Old,
}

#[derive(Debug, Clone, Copy)]
enum BinOp {
    Mult(Var),
    Plus(Var),
}

impl BinOp {
    fn eval(&self, old: i64) -> i64 {
        match *self {
            BinOp::Mult(Var::Literal(x)) => old * x,
            BinOp::Mult(Var::Old) => old * old,
            BinOp::Plus(Var::Literal(x)) => old + x,
            BinOp::Plus(Var::Old) => old + old,
        }
    }
}

impl FromStr for BinOp {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, var) = s
            .strip_prefix("old ")
            .ok_or(())?
            .split_once(' ')
            .ok_or(())?;

        let var = match var {
            "old" => Var::Old,
            num => Var::Literal(num.parse::<i64>().map_err(|_| ())?),
        };

        match op {
            "+" => Ok(BinOp::Plus(var)),
            "*" => Ok(BinOp::Mult(var)),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    op: BinOp,
    test_div: i64,
    tf: (usize, usize),
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        lines
            .next()
            .ok_or(())?
            .strip_prefix("Monkey ")
            .and_then(|s| s.strip_prefix(|c: char| c.is_ascii_digit()))
            .and_then(|s| (s == ":").then_some(s))
            .ok_or(())?;

        let items: Vec<_> = lines
            .next()
            .ok_or(())?
            .strip_prefix("  Starting items: ")
            .ok_or(())?
            .split(", ")
            .flat_map(|s| s.parse::<i64>().map_err(|_| ()))
            .collect();

        let op = lines
            .next()
            .ok_or(())?
            .strip_prefix("  Operation: new = ")
            .and_then(|op_s| op_s.parse::<BinOp>().ok())
            .ok_or(())?;

        let test_div = lines
            .next()
            .ok_or(())?
            .strip_prefix("  Test: divisible by ")
            .and_then(|div_s| div_s.parse::<i64>().ok())
            .ok_or(())?;

        let t = lines
            .next()
            .ok_or(())?
            .strip_prefix("    If true: throw to monkey ")
            .and_then(|t| t.parse::<usize>().ok())
            .ok_or(())?;

        let f = lines
            .next()
            .ok_or(())?
            .strip_prefix("    If false: throw to monkey ")
            .and_then(|f| f.parse::<usize>().ok())
            .ok_or(())?;

        Ok(Monkey {
            items,
            op,
            test_div,
            tf: (t, f),
        })
    }
}

fn extract_tf_mut(
    m: &mut Vec<Monkey>,
    i: usize,
    j: usize,
    k: usize,
) -> Option<(&mut Monkey, &mut Monkey, &mut Monkey)> {
    if i != j && j != k && i != k {
        assert!(i < m.len());
        assert!(j < m.len());
        assert!(k < m.len());
        // safety:
        //   non-aliasing indices mean that mut refs cann be taken to all 3
        //   similar to slice.split_at_mut
        Some(unsafe {
            (
                m.as_mut_ptr().add(i).as_mut().unwrap_unchecked(),
                m.as_mut_ptr().add(j).as_mut().unwrap_unchecked(),
                m.as_mut_ptr().add(k).as_mut().unwrap_unchecked(),
            )
        })
    } else {
        None
    }
}

// euclidian gcd
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let fpath = std::env::args().nth(1).expect("no file argument found");

    let mut input = String::new();
    BufReader::new(File::open(fpath)?).read_to_string(&mut input)?;

    let mut monkeys: Vec<_> = input
        .split("\n\n")
        .flat_map(|s| s.parse::<Monkey>())
        .collect();

    let mut monkeys_2 = monkeys.clone();

    let mut inspects: Vec<u64> = vec![0; monkeys.len()];
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let (j, k) = monkeys[i].tf;
            let (m, t, f) = extract_tf_mut(&mut monkeys, i, j, k)
                .unwrap_or_else(|| panic!("monkey {i} cant pass to self"));

            inspects[i] += m.items.len() as u64;

            m.items
                .drain(..)
                .map(|wl| m.op.eval(wl) / 3)
                .for_each(|wl| {
                    if wl % m.test_div == 0 {
                        t.items.push(wl);
                    } else {
                        f.items.push(wl);
                    }
                });
        }
    }

    let i = inspects.len() - 2;
    inspects.select_nth_unstable(i);
    println!(
        "Part 1: {}",
        inspects[inspects.len() - 2] * inspects[inspects.len() - 1]
    );

    let gcd = monkeys_2.iter().map(|m| m.test_div).fold(0, gcd);
    let lcm: i64 = monkeys_2.iter().map(|m| m.test_div).product::<i64>() / gcd;

    let mut inspects: Vec<u64> = vec![0; monkeys_2.len()];
    for _ in 0..10_000 {
        for i in 0..monkeys_2.len() {
            let (j, k) = monkeys_2[i].tf;
            let (m, t, f) = extract_tf_mut(&mut monkeys_2, i, j, k)
                .unwrap_or_else(|| panic!("monkey {i} cant pass to self"));

            inspects[i] += m.items.len() as u64;

            m.items.drain(..).map(|wl| m.op.eval(wl)).for_each(|wl| {
                let rem = wl % m.test_div;
                if rem == 0 {
                    t.items.push(wl % lcm);
                } else {
                    f.items.push(wl % lcm);
                }
            });
        }
    }

    let i = inspects.len() - 2;
    inspects.select_nth_unstable(i);
    println!(
        "Part 2: {}",
        inspects[inspects.len() - 2] * inspects[inspects.len() - 1]
    );

    Ok(())
}

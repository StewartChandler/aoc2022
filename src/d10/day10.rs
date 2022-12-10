use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};


fn main() -> Result<(), Box<dyn Error>> {
    let fpath = std::env::args().nth(1).expect("no file argument found");

    let lines = BufReader::new(File::open(fpath)?)
        .lines()
        .flatten()
        .filter(|s| !s.is_empty());

    let mut x_vals = Vec::new();
    lines.for_each({
        let mut x = 1; 
        let vref = &mut x_vals; 
        move |s| {
            let (l, r) = s.split_at(4);
            match l {
                "noop" => { vref.push(x) },
                "addx" => { 
                    vref.push(x);
                    vref.push(x);
                    x += r.trim().parse::<i32>().unwrap(); 
                },
                _ => {panic!("Unknown instruction {l}")}
            };
    }});

    let res = (20..=220).step_by(40).map(|i| x_vals[i - 1] * (i as i32)).sum::<i32>();

    println!("Part 1: {res}");

    print!("Part 2: ");
    (0..6).flat_map(|y| (0..40).zip(std::iter::repeat(y))).for_each(|(x, y)| {
        let c = y * 40 + x;
        let m = if x_vals[c] > 0 { 7u64.wrapping_shl((x_vals[c] - 1) as u32)} else { 7u64.wrapping_shr((1 - x_vals[c]) as u32) };

        let s = if ((1 << x) & m) != 0 {
            "#"
        } else {
            "."
        };

        if x == 0 && y != 0 {
            print!("        {s}");
        } else if x == 39 {
            println!("{s}");
        } else {
            print!("{s}");
        }
    });

    Ok(())
}
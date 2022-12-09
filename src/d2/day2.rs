use std::{
    error::Error,
    fmt::{Display, Formatter},
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug, Clone, Copy)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
enum Score {
    Win,
    Tie,
    Loss,
}

#[derive(Debug)]
struct Strategy {
    abc: Rps,
    xyz: Rps,
}

impl Strategy {
    fn score(&self) -> Score {
        match (self.abc, self.xyz) {
            (Rps::Rock, Rps::Rock) => Score::Tie,
            (Rps::Rock, Rps::Paper) => Score::Win,
            (Rps::Rock, Rps::Scissors) => Score::Loss,
            (Rps::Paper, Rps::Rock) => Score::Loss,
            (Rps::Paper, Rps::Paper) => Score::Tie,
            (Rps::Paper, Rps::Scissors) => Score::Win,
            (Rps::Scissors, Rps::Rock) => Score::Win,
            (Rps::Scissors, Rps::Paper) => Score::Loss,
            (Rps::Scissors, Rps::Scissors) => Score::Tie,
        }
    }
}

#[derive(Debug)]
struct StrategyParseError {
    e: &'static str,
}

impl Display for StrategyParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.e)
    }
}
impl Error for StrategyParseError {}

impl FromStr for Strategy {
    type Err = StrategyParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s0, s1) = s
            .split_once(char::is_whitespace)
            .ok_or(StrategyParseError { e: "invaild input" })?;

        Ok(Strategy {
            abc: match s0.parse::<char>() {
                Ok('A') => Rps::Rock,
                Ok('B') => Rps::Paper,
                Ok('C') => Rps::Scissors,
                Err(_) | Ok(_) => return Err(StrategyParseError { e: "invalid char" }),
            },
            xyz: match s1.parse::<char>() {
                Ok('X') => Rps::Rock,
                Ok('Y') => Rps::Paper,
                Ok('Z') => Rps::Scissors,
                Err(_) | Ok(_) => return Err(StrategyParseError { e: "invalid char" }),
            },
        })
    }
}

struct RevStrategy {
    abc: Rps,
    xyz: Score,
}

impl RevStrategy {
    fn rps(&self) -> Rps {
        match (self.abc, self.xyz) {
            (Rps::Rock, Score::Tie) => Rps::Rock,
            (Rps::Rock, Score::Win) => Rps::Paper,
            (Rps::Rock, Score::Loss) => Rps::Scissors,
            (Rps::Paper, Score::Loss) => Rps::Rock,
            (Rps::Paper, Score::Tie) => Rps::Paper,
            (Rps::Paper, Score::Win) => Rps::Scissors,
            (Rps::Scissors, Score::Win) => Rps::Rock,
            (Rps::Scissors, Score::Loss) => Rps::Paper,
            (Rps::Scissors, Score::Tie) => Rps::Scissors,
        }
    }
}

impl FromStr for RevStrategy {
    type Err = StrategyParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s0, s1) = s
            .split_once(char::is_whitespace)
            .ok_or(StrategyParseError { e: "invaild input" })?;

        Ok(RevStrategy {
            abc: match s0.parse::<char>() {
                Ok('A') => Rps::Rock,
                Ok('B') => Rps::Paper,
                Ok('C') => Rps::Scissors,
                Err(_) | Ok(_) => return Err(StrategyParseError { e: "invalid char" }),
            },
            xyz: match s1.parse::<char>() {
                Ok('X') => Score::Loss,
                Ok('Y') => Score::Tie,
                Ok('Z') => Score::Win,
                Err(_) | Ok(_) => return Err(StrategyParseError { e: "invalid char" }),
            },
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fpath = std::env::args().nth(1).expect("no file argument found");
    let mut rounds: Vec<Strategy> = Vec::new();
    let mut rounds2: Vec<RevStrategy> = Vec::new();
    for s in BufReader::new(File::open(fpath)?).lines().flatten() {
        rounds.push(s.parse()?);
        rounds2.push(s.parse()?);
    }

    // eprintln!("{:?}", rounds);

    // part 1
    println!(
        "Part 1: {}",
        rounds
            .into_iter()
            .map(|strat| match strat.score() {
                Score::Win => 6,
                Score::Tie => 3,
                Score::Loss => 0,
            } + match strat.xyz {
                Rps::Rock => 1,
                Rps::Paper => 2,
                Rps::Scissors => 3,
            })
            .sum::<u32>()
    );

    // part 2
    println!(
        "Part 2: {}",
        rounds2
            .into_iter()
            .map(|rstrat| match rstrat.rps() {
                Rps::Rock => 1,
                Rps::Paper => 2,
                Rps::Scissors => 3,
            } + match rstrat.xyz {
                Score::Win => 6,
                Score::Tie => 3,
                Score::Loss => 0,
            })
            .sum::<u32>()
    );

    Ok(())
}

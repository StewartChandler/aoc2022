# Advent of Code 2022

This is a repository for my solutions to [Advent of Code](https://adventofcode.com/) for 2022. This, year I am writing my solutions in Rust.  Over the summer of 2022, I started learning and messing around in Rust, and I thought that this AoC would provide a great opportunity to gain practice in the language.

## Running the Code.

This code is managed through cargo, where each day is its own binary titled `day[N]` with source file `src/d[N]/day[N].rs` where `[N]` is the day of the month.  Each binary expects a single argument to be passed that is the name of a file to use as input.

You can use cargo to build and run the binary for a given day
```bash
# build day1 
cargo build --bin day1

# build and run day1 with input file src/d1/input.txt
crago run --bin day1 -- src/d1/input1.txt
```
In this case `src/d1/input1.txt` is the example provided by the AoC page. And the result of the latter is:
```bash
Part 1: 24000
Part 2: 45000
```
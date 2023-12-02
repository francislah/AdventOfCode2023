extern crate aoc2023;
use aoc2023::my_lib;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    let file = my_lib::open_file("src/bin/day2/input.txt");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(e) => return Err(e)
        }
    }
    Ok(())
}
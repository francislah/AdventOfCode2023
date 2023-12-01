use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    let file = match File::open("input.txt") {
        Ok(file) => file,
        Err(e) => return Err(e)
    };
    let reader = BufReader::new(file);
    let mut entire_calibration = 0;
    for line in reader.lines() {
        match line {
            Ok(line) => entire_calibration += get_calibration(&line),
            Err(e) => return Err(e)
        }
    }
    println!("{}", entire_calibration);
    Ok(())
}

fn search_numbers(line : &str) -> i32 {
    let numbers = vec!["one", "two", "three", "four", "five", "six",
                       "seven", "eight", "nine"];
    for (index, number) in numbers.iter().enumerate() {
        if line.starts_with(number) {
            return index as i32 + 1
        }
    }
    0
}
fn get_calibration(line: &str) -> i32 {
    let mut calibration = 0;
    let len = line.len();
    for (i, c) in line.chars().enumerate() {
        let tmp = search_numbers(&line[i..]);
        if tmp > 0 {
            calibration = tmp * 10;
            break;
        }
        if c.is_numeric() {
            calibration += (c as i32 - 48) * 10;
            break;
        }
    }
    for (i, c) in line.chars().rev().enumerate() {
        let tmp =  search_numbers(&line[(len - i)..]);
        if tmp > 0 {
            calibration += tmp;
            break;
        }
        if c.is_numeric() {
            calibration += c as i32 - 48;
            break;
        }
    }
    calibration
}

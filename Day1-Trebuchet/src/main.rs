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

fn get_calibration(line: &str) -> i32 {
    let mut calibration = 0;
    for c in line.chars() {
        if c.is_numeric() {
            calibration += (c as i32 - 48) * 10;
            break;
        }
    }
    for c in line.chars().rev() {
        if c.is_numeric() {
            calibration += c as i32 - 48;
            break;
        }
    }
    calibration
}

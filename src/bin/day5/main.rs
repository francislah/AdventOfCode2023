extern crate aoc2023;

use aoc2023::my_lib::{open_file, read_file_into_vec};

struct AlmanacItem {
    seed: i32,
    soil: i32,
    fertilizer: i32,
    water: i32,
    light: i32,
    temperature: i32,
    humidity: i32,
    location: i32
}

fn main() {
    let file = open_file("src/bin/day5/test-input.txt");
    let lines: Vec<String> = read_file_into_vec(&file);
    lines.iter().for_each(|line| println!("{}", line));
}
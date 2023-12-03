extern crate aoc2023;

use std::collections::HashMap;
use std::hash::Hash;
use std::io::{prelude::*, BufReader};
use aoc2023::my_lib;

#[derive(Default, Debug)]
struct Strings {
    row: usize,
    prev: String,
    curr: String,
    next: String
}
fn main() {
    let file = my_lib::open_file("src/bin/day3/input.txt");
    let reader = BufReader::new(file);
    let mut gears: HashMap<(usize, usize), (usize, i32)> = HashMap::new();
    let schema: Vec<String> = reader.lines().map(|line| {
        match line {
            Ok(line) => line,
            Err(e) => panic!("{}", e)
        }
    }).collect();
    let mut schema_lines = Strings{ ..Default::default() };
    let mut total = 0;
    for (i, line) in schema.iter().enumerate() {
        schema_lines.prev = schema_lines.curr;
        schema_lines.curr = schema_lines.next;
        schema_lines.next = line.clone();
        if schema_lines.curr.len() > 0 {
            schema_lines.row = i - 1;
            total += verify(&schema_lines, &mut gears);
        }
    }
    schema_lines.row += 1;
    schema_lines.prev = schema_lines.curr;
    schema_lines.curr = schema_lines.next;
    schema_lines.next = "".to_string();
    total += verify(&schema_lines, &mut gears);
    println!("Total is {}", total);
    println!("Total value for gears is {}", gears.values().filter(|v| v.0 == 2).map(|v| v.1).sum::<i32>());
}

fn verify(schema_lines: &Strings, gears: &mut HashMap<(usize, usize), (usize, i32)>) -> i32 {
    let mut line_total = 0;
    let mut num = 0;
    let mut to_verify: bool = false;
    let mut pos = (0usize, 0usize);
    for (i, c) in schema_lines.curr.chars().enumerate() {
        if c.is_numeric() {
            if num == 0 { pos.0 = i }
            num = num * 10 + c as i32 - 48;
            pos.1 = i;
        }
        else if num > 0 {
            to_verify = true
        }
        if to_verify || (i == schema_lines.curr.len() - 1 && num != 0) {
            let start = if pos.0 > 0 { pos.0 - 1 } else { 0 };
            let end = if pos.1 < schema_lines.curr.len() - 1 { pos.1 + 1 } else { pos.1 };
            if verify_curr(&schema_lines, start, end, gears, num) {
                line_total += num;
            }
            num = 0;
        }
        to_verify = false;
    }
    line_total
}

#[allow(dead_code)]
fn display_surroundings(schema: &Strings, start: usize, end: usize) {
    if schema.prev.len() > 0 {
        println!("{}", &schema.prev[start..=end]);
    }
    if schema.curr.len() > 0 {
        println!("{}", &schema.curr[start..=end]);
    }
    if schema.next.len() > 0 {
        println!("{}", &schema.next[start..=end]);
    }
}

fn verify_curr(schema: &Strings, start: usize, end: usize, gears: &mut HashMap<(usize, usize), (usize, i32)>, num : i32) -> bool {
    // display_surroundings(schema, start, end);
    let mut valid: bool;
    if !schema.curr[start..=end].starts_with(".") || !schema.curr[start..=end].ends_with(".") {
        valid = find_symbol(&schema.curr[start..=end], gears, schema.row, start, num);
        if valid { return true };
    }
    if schema.prev.len() > 0 {
        valid = find_symbol(&schema.prev[start..=end], gears, schema.row - 1, start, num);
        if valid { return valid };
    }
    if schema.next.len() > 0 {
        valid = find_symbol(&schema.next[start..=end], gears, schema.row + 1, start, num);
        if valid { return valid };
    }
    false
}

fn find_symbol(line: &str, gears: &mut HashMap<(usize, usize), (usize, i32)>, row: usize, pos:usize, num: i32) -> bool {
    for (i, c) in line.chars().enumerate() {
        if c.is_numeric() || c == '.' {
            continue
        } else {
            if c == '*' {
                match gears.get(&(row, (pos + i))) {
                    Some(v) => {
                        gears.insert((row, pos + i),(2, v.1 * num))
                    },
                    None => gears.insert((row, pos + i), (1, num))
                };
            }
            return true
        }
    }
    false
}
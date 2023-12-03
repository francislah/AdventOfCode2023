extern crate aoc2023;

use std::io::{prelude::*, BufReader};
use aoc2023::my_lib;

#[derive(Default, Debug)]
struct Strings {
    prev: String,
    curr: String,
    next: String
}
fn main() {
    let file = my_lib::open_file("src/bin/day3/input.txt");
    let reader = BufReader::new(file);
    let schema: Vec<String> = reader.lines().map(|line| {
        match line {
            Ok(line) => line,
            Err(e) => panic!("{}", e)
        }
    }).collect();
    let mut schema_lines = Strings{ ..Default::default() };
    let mut total = 0;
    for line in schema.iter() {
        schema_lines.prev = schema_lines.curr;
        schema_lines.curr = schema_lines.next;
        schema_lines.next = line.clone();
        if schema_lines.curr.len() > 0 {
            total += verify(&schema_lines);
        }
        // println!("index {}: {}", i, line);
    }
    schema_lines.prev = schema_lines.curr;
    schema_lines.curr = schema_lines.next;
    schema_lines.next = "".to_string();
    total += verify(&schema_lines);
    println!("Total is {}", total);
}

fn verify(schema_lines: &Strings) -> i32 {
    let mut line_total = 0;
    let mut num = 0;
    let mut to_verify: bool = false;
    let mut pos = (0usize, 0usize);
    for (i, c) in schema_lines.curr.chars().enumerate() {

        // if i > 0 && i == schema_lines.curr.len() - 1 { to_verify = true }
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
            if verify_curr(&schema_lines, start, end) {
                line_total += num;
                // println!("{}: true", num);
            } else {
                // println!("{}: false", num);
            }
            num = 0;
        }
        to_verify = false;
            // println!("index: {} with char: {} and len {}", i, c, schema_lines.curr.len() )
        // } else if num > 0 {
        //     to_verify = true;
        // }
        // if to_verify == true {
        //     num = 0;
        //     to_verify = false
        // }
        // println!("{}", i);
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

fn verify_curr(schema: &Strings, start: usize, end: usize) -> bool {
    // display_surroundings(schema, start, end);
    let mut valid: bool;
    if !schema.curr[start..=end].starts_with(".") || !schema.curr[start..=end].ends_with(".") {
        for c in schema.curr[start..=end].chars() {
            if !c.is_numeric() && c != '.' {
                return true
            }
        }
    }
    if schema.prev.len() > 0 {
        valid = find_symbol(&schema.prev[start..=end]);
        if valid { return valid };
    }
    if schema.next.len() > 0 {
        valid = find_symbol(&schema.next[start..=end]);
        if valid { return valid };
    }
    false
}

fn find_symbol(line: &str) -> bool {
    for c in line.chars() {
        if c.is_numeric() || c == '.' {
            continue
        } else {
            return true
        }
    }
    false
}
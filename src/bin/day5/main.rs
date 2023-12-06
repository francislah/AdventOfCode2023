extern crate aoc2023;

use std::collections::HashMap;
use aoc2023::my_lib::{open_file, read_file_into_vec};

#[derive(Default)]
struct AlmanacItem {
    soil: i32,
    fertilizer: i32,
    water: i32,
    light: i32,
    temperature: i32,
    humidity: i32,
    location: i32
}

struct ItemData {
    start: Vec<i32>,
    range: Vec<i32>,
    dest: Vec<i32>
}

enum Item {
    SOIL,
    FERTILIZER,
    WATER,
    LIGHT,
    TEMPERATURE,
    HUMIDITY,
    LOCATION
}

fn main() {
    let file = open_file("src/bin/day5/test-input.txt");
    let lines: Vec<String> = read_file_into_vec(&file);
    let mut seeds: Vec<i32> = vec![];
    let mut almanac_items: HashMap<i32, AlmanacItem> = HashMap::new();
    let mut current: Item = Item::SOIL;
    for line in lines {
        if line.starts_with("seeds: ") {
            add_seeds(&line[7..], &mut seeds);
            for seed in &seeds {
                almanac_items.insert(*seed, AlmanacItem{..Default::default()});
            }
        } else if line.ends_with(":") {
            if let Some(item) = line.trim_end_matches(':')
                .split('-')
                .nth(2)
                .and_then(|word| word.split_once(' ')) {
                    current = set_current(&item.0)
                };

        } else if line.trim() == "" { continue; }
        else {
            let mut vec: Vec<i32> = vec![];
            line.split_whitespace().for_each(|n| match n.parse::<i32>() {
                Ok(n) => {println!("{}", n); vec.push(n)},
                Err(e) => panic!("{}: {}", e, n)
            });
            println!("{}", match vec.iter().next() {Some(n) => n, None => &0});
            // if vec.len() == 0 {println!("vec 0 = {}", line)}
            // else println!()
            println!("to look s: {}", vec[1]);
        }
    };

}

fn set_current (item: &str) -> Item {
    match item {
        "soil" => Item::SOIL,
        "fertilizer" => Item::FERTILIZER,
        "water" => Item::WATER,
        "light" => Item::LIGHT,
        "temperature" => Item::TEMPERATURE,
        "humidity" => Item::HUMIDITY,
        "location" => Item::LOCATION,
        _ => panic!("Error: wrong item ({})", item)
    }
}

fn add_seeds(numbers: &str, seeds: &mut Vec<i32>) {
    // println!("{}", numbers);
    numbers.split_whitespace().for_each(|n| {
        seeds.push(match n.parse::<i32>() {
            Ok(n) => n,
            Err(e) => panic!("{}", e)
        })
    })
}
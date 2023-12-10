extern crate aoc2023;

use aoc2023::my_lib;
use std::collections::HashMap;
use std::slice::Iter;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Item {
    SEED,
    SOIL,
    FERTILIZER,
    WATER,
    LIGHT,
    TEMPERATURE,
    HUMIDITY,
    LOCATION,
}

fn main() {
    let file = my_lib::open_file("src/bin/day5/input.txt");
    let lines = my_lib::read_file_into_vec(&file);
    let mut almanac: HashMap<Item, (Vec<i64>, Vec<i64>)> = HashMap::new();
    let mut context = (Item::SEED, Item::SEED);
    for line in &lines {
        // println!("line: {}", line);
        if line.starts_with("seeds:") {
            let seeds_setting = get_num_vec(&line[6..]);
            let mut seeds: Vec<i64> = vec![];
            let mut start: i64 = 0;
            let mut qty: i64 = 0;
            for (i, v) in seeds_setting.iter().enumerate() {
                if i % 2 == 0 {
                    start = *v;
                } else {
                    qty = *v;
                    for x in 0..qty {
                        // println!("start: {}", start);
                        seeds.push(start);
                        start += 1;
                    }
                }
            }
            // println!("{:?}", seeds);
            almanac
                .entry(context.1)
                .or_insert_with(|| (seeds.clone() , vec![]));
        } else if line.is_empty() {
            continue;
        } else if line.ends_with(":") {
            match almanac.get_mut(&context.1) {
                Some((src, dst)) => {
                    for i in src {
                        // println!("pushing {}", *i);
                        dst.push(*i);
                    }
                }
                None => panic!("Error"),
            }
            context = get_context(&mut context, &line.trim_end_matches(" map:"));
            match almanac.get(&context.0) {
                Some((src, dst)) => almanac.insert(context.1, (dst.clone(), vec![])),
                None => panic!("Error"),
            };
            // println!("context 0: {:?}, context 1: {:?}", context.0, context.1)
        } else {
            let mut setting = get_setting(get_num_vec(&line));
            let mut to_verify = match almanac.get(&context.1) {
                Some((src, dst)) => src.clone(),
                None => panic!("Error"),
            };
            // println!("setting: {:?}", setting);
            // println!("verify: {:?}", to_verify);
            for (i, item) in to_verify.iter().enumerate() {
                if item >= &setting.1 && item <= &(setting.1 + setting.2) {
                    match almanac.get_mut(&context.1) {
                        Some((src, dst)) => {
                            // println!("index: {}, with {}", i, item);
                            if let Some(index) = src.iter().position(|value| *value == *item) {
                                src.swap_remove(index);
                            }
                            dst.push(setting.0 + item - setting.1)
                        }
                        None => panic!("{}", "Error"),
                    }
                }
            }
        }
    }
    match almanac.get_mut(&context.1) {
        Some((src, dst)) => {
            for i in src {
                // println!("pushing {}", *i);
                dst.push(*i);
            }
        }
        None => panic!("Error"),
    }
   let location_min = match almanac.get(&Item::LOCATION) {
        Some((src, dst)) => match dst.iter().min() {
            Some(v) => v,
            None => &0
        },
       None => panic!("Error")
    };
    println!("{}", location_min);
}

fn get_setting(settings: Vec<i64>) -> (i64, i64, i64) {
    let src = settings[0];
    let dst = settings[1];
    let range = settings[2];
    (src, dst, range)
}
fn get_context(context: &mut (Item, Item), from_to: &str) -> (Item, Item) {
    let first_end = match from_to.find('-') {
        Some(pos) => pos,
        None => 0,
    };
    let context_1 = get_item_enum(&from_to[..first_end]);
    let second_start = match from_to.rfind('-') {
        Some(pos) => pos,
        None => 0,
    };
    let context_2 = get_item_enum(&from_to[second_start + 1..]);
    (context_1, context_2)
}

fn get_item_enum(item: &str) -> Item {
    match item {
        "seed" => Item::SEED,
        "soil" => Item::SOIL,
        "fertilizer" => Item::FERTILIZER,
        "water" => Item::WATER,
        "light" => Item::LIGHT,
        "temperature" => Item::TEMPERATURE,
        "humidity" => Item::HUMIDITY,
        "location" => Item::LOCATION,
        _ => panic!("Invalid context: {}", item),
    }
}

fn get_num_vec(seed_line: &str) -> Vec<i64> {
    let mut vec: Vec<i64> = vec![];
    let numbers = seed_line.split_whitespace();
    for num in numbers {
        vec.push(match num.parse::<i64>() {
            Ok(n) => n,
            Err(_) => {
                println!("{}", num);
                0
            }
        })
    }
    vec
}

extern crate aoc2023;

use std::collections::HashMap;
use std::slice::Iter;
use aoc2023::my_lib;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Item {
    SEED,
    SOIL,
    FERTILIZER,
    WATER,
    LIGHT,
    TEMPERATURE,
    HUMIDITY,
    LOCATION
}

struct Setting {
    dst: i32,
    src: i32,
    range: i32
}

fn main() {
    let file = my_lib::open_file("src/bin/day5/test-input.txt");
    let lines = my_lib::read_file_into_vec(&file);
    let mut almanac: HashMap<Item, Vec<i32>> = HashMap::new();
    let mut context = (Item::SEED, Item::SEED);
    for line in &lines {
        if line.starts_with("seeds:") {
            almanac.entry(context.1).or_insert_with(|| get_num_vec(&line));
        } else if line.is_empty() {
            continue
        } else if line.ends_with(":") {
            context = get_context(&mut context, &line.trim_end_matches(" map:"));
        } else {
            let to_calculate = get_setting(get_num_vec(&line));

            almanac.insert(Item::SEED, get_num_vec(&line));
            // add_to_almanac(&mut almanac, &line, context);
            println!("deal with this: {}", line);
        }
    }

    // set_all_soils(&mut almanac, &mut lines.iter());

    // for line in lines {
    //     println!("{}", line);
    // }
    println!("{:?}", almanac)
}

fn get_setting(settings: Vec<i32>) -> Setting {
    let src = settings[0];
    let dst = settings[1];
    let range = settings[2];
    Setting{src, dst, range}
}
fn get_context(context: &mut (Item, Item), from_to: &str) -> (Item, Item) {
    let first_end = match from_to.find('-') {
        Some(pos) => pos,
        None => 0
    };
    let context_1 = get_item_enum(&from_to[..first_end]);
    let second_start = match from_to.rfind('-') {
        Some(pos) => pos,
        None => 0
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
        _ => panic!("Invalid context: {}", item)
    }
}

fn get_num_vec(seed_line: &String) -> Vec<i32> {
    let mut vec: Vec<i32> = vec![];
    let numbers = seed_line.split_whitespace();
    for num in numbers {
        vec.push(match num.parse::<i32>(){
            Ok(n) => n,
            Err(_) => 0
        })
    }
    vec
}
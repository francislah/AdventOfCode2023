extern crate aoc2023;
use aoc2023::my_lib;
use std::io::{prelude::*, BufReader};

#[derive(Default)]
struct Bag{
    blue: i32,
    red: i32,
    green: i32
}

fn main() -> std::io::Result<()> {
    let file = my_lib::open_file("src/bin/day2/input.txt");
    let reader = BufReader::new(file);
    let mut power = 0;
    let game_ids: i32 = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| get_id(&line, &mut power))
        .sum();
    println!("Sum of Game Ids = {}", game_ids);
    println!("Power of all games = {}", power);
    Ok(())
}

fn get_amount(color: &str) -> i32 {
    color.parse::<i32>().unwrap_or_else(|e| {
        eprintln!("{}", e);
        panic!("Error")
    })
}

fn get_color(color: &str) -> usize {
    match color {
        "blue" => 0,
        "red" => 1,
        "green" => 2,
        _ => panic!("Not a valid color")
    }
}

fn verify_rule(bag: &Bag, power: &mut i32) -> bool {
    let rule = Bag{blue: 14, green: 13, red: 12};
    *power += bag.blue * bag.red * bag.green;
    return bag.blue <= rule.blue && bag.red <= rule.red && bag.green <= rule.green;
}

fn get_id(line: &str, power: &mut i32) -> i32 {
    let mut bag: Bag = Default::default();
    if line.starts_with("Game ") {
        let end = match line.find(":") {
            Some(i) => i,
            None => return 0
        };
        let mut tmp_bag: Bag = Default::default();
        line[(end + 1)..].split(";").for_each(|turn| -> () {
            let mut turn_bag: Bag = Default::default();
            turn.split(",").for_each(|color| -> () {
                let mut amount = 0;
                for (i, split) in color.trim().split(" ").enumerate() {
                    if i == 0 { amount = get_amount(split)}
                    else {
                        match get_color(split) {
                            0 => turn_bag.blue += amount,
                            1 => turn_bag.red += amount,
                            2 => turn_bag.green += amount,
                            _ => panic!("How did it get here?")
                        }
                    }
                }
            });
            if turn_bag.blue > tmp_bag.blue { tmp_bag.blue = turn_bag.blue }
            if turn_bag.red > tmp_bag.red { tmp_bag.red = turn_bag.red }
            if turn_bag.green > tmp_bag.green { tmp_bag.green = turn_bag.green }
        });
        if tmp_bag.blue > bag.blue { bag.blue = tmp_bag.blue }
        if tmp_bag.green > bag.green { bag.green = tmp_bag.green }
        if tmp_bag.red > bag.red { bag.red = tmp_bag.red }
        if verify_rule(&bag, power) {
            return match line[5..end].parse::<i32>() {
                Ok(n) => n,
                Err(e) => { println!("{}", e); 0}
            };
        }
    }
    0
}
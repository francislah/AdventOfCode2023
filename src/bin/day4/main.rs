extern crate aoc2023;

use std::collections::HashMap;
use std::io::{prelude::*, BufReader};
use aoc2023::my_lib;

fn fill_vec(line: &str, vec: &mut Vec<i32>) {
    line.split_whitespace().for_each(|n| vec.push(match n.parse::<i32>() {
        Ok(n) => n,
        _ => 0
    }));
}
fn get_card_points(line: &str, index: usize, cards: &mut HashMap<usize, usize>) -> i32 {
    let card: (&str, &str) = match line.find(":") {
        Some(start) => match line[start + 1..].split_once("|") {
            Some((f,s)) => (f,s),
            None => panic!("Card invalid")
        },
        None => panic!("Card invalid")
    };
    let mut results: Vec<i32> = vec![];
    let mut draws: Vec<i32> = vec![];
    let mut total = 0;
    fill_vec(card.0, &mut results);
    fill_vec(card.1, &mut draws);
    let mut count: usize = 0;
    for result in results {
        if draws.contains(&result) {
            count += 1;
            if total > 0 { total *= 2} else { total = 1 };
        }
    }
    let n_cards: usize = match cards.get(&(index)) {
        Some(v) => *v as usize,
        None => {
            cards.insert(index, 1);
            1
        }
    };
    for x in 1..=count {
        match cards.get(&(index + x)) {
            Some(v) => {
                cards.insert(index + x, n_cards + v);
            },
            None => {
                cards.insert(index + x, n_cards + 1);
            }
        };
    }
    total
}

fn main() {
    let file = my_lib::open_file("src/bin/day4/input.txt");
    let reader = BufReader::new(file);
    let mut cards: HashMap<usize, usize> = HashMap::new();
    let total_points = reader.lines().enumerate().map(|(i, line) | {
        match line {
            Ok(line) => get_card_points(&line, i, &mut cards),
            Err(e) => panic!("{}", e)
        }
    }).sum::<i32>();
    println!("Total points: {}", total_points);
    println!("Total cards: {}", cards.values().map(|v| *v).sum::<usize>())
}
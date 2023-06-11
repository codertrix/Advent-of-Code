// ---------------------------------------------------
//  Advent of Code 2016
//  Day 10: Balance Bots
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashMap;
use std::mem::swap;

const INPUT: &str = include_str!("day10.in");

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n----------------------");
    println!(" Day 10: Balance Bots");
    println!("----------------------");

    let (result_one, result_two) = puzzle_one_two();

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two() -> (usize, usize) {
    let mut bots = HashMap::new();
    let mut outputs = Vec::new();
    let mut instructions = Vec::new();

    for line in INPUT.lines() {
        if line.starts_with("value") {
            let parts: Vec<_> = line.split_whitespace().collect();
            let bot: usize = parts[5].parse().unwrap();
            let value: usize = parts[1].parse().unwrap();

            bots.entry(bot).or_insert(Vec::new()).push(value);
        } else {
            let parts: Vec<_> = line.split_whitespace().collect();
            let bot: usize = parts[1].parse().unwrap();
            let low: usize = parts[6].parse().unwrap();
            let high: usize = parts[11].parse().unwrap();

            instructions.push((bot, parts[5] == "bot", low, parts[10] == "bot", high));
        }
    }

    let mut result_one = None;

    'outer: loop {
        for instruction in instructions.iter() {
            if let Some(values) = bots.get_mut(&instruction.0) {
                if values.len() == 2 {
                    let mut high = values.pop().unwrap();
                    let mut low = values.pop().unwrap();

                    if low > high {
                        swap(&mut low, &mut high);
                    }

                    if low == 17 && high == 61 {
                        result_one = Some(instruction.0);
                    }

                    if instruction.1 {
                        bots.entry(instruction.2).or_insert(Vec::new()).push(low);
                    } else if instruction.2 < 3 {
                        outputs.push(low);
                    }

                    if instruction.3 {
                        bots.entry(instruction.4).or_insert(Vec::new()).push(high);
                    } else if instruction.4 < 3 {
                        outputs.push(high);
                    }

                    if outputs.len() == 3 && result_one.is_some() {
                        break 'outer;
                        
                    }
                }
            }
        }
    };

    (result_one.unwrap(), outputs.into_iter().product())
}

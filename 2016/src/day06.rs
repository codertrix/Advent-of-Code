// ---------------------------------------------------
//  Advent of Code 2016
//  Day 6: Signals and Noise
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashMap;

const INPUT: &str = include_str!("day06.in");

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n--------------------------");
    println!(" Day 6: Signals and Noise");
    println!("--------------------------");

    let (result_one, result_two) = puzzle_one_two();

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two() -> (String, String) {
    let mut characters = Vec::new();

    for _ in 0..8 {
        characters.push(HashMap::new());
    }

    for line in INPUT.lines() {
        for (idx, c) in line.chars().enumerate() {
            *characters[idx].entry(c).or_insert(0) += 1;
        }
    }

    let mut message_one = String::new();
    let mut message_two = String::new();

    for character_map in characters {
        message_one.push(*character_map.iter().max_by_key(|(_, v)| *v).unwrap().0);
        message_two.push(*character_map.iter().min_by_key(|(_, v)| *v).unwrap().0);
    }

    (message_one, message_two)
}

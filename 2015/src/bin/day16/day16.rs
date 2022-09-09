// ---------------------------------------------------
//  Advent of Code 2015
//  Day 16: Aunt Sue
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashMap;

const INPUT: &str = include_str!("day16.in");
const MESSAGE: &str = include_str!("message.in");

pub fn run() {
    let mut message = HashMap::new();

    for line in MESSAGE.lines() {
        let parts: Vec<_> = line
            .split([' ', ':'])
            .collect();

        message.insert(parts[0], parts[2].parse().unwrap_or(0));
    }

    let mut sues = Vec::new();

    for line in INPUT.lines() {
        let mut sue = HashMap::new();
        let parts: Vec<_> = line
            .split([' ', ':', ','])
            .collect();

        sue.insert(parts[3], parts[5].parse().unwrap_or(0));
        sue.insert(parts[7], parts[9].parse().unwrap_or(0));
        sue.insert(parts[11], parts[13].parse().unwrap_or(0));
        sues.push(sue);
    }

    println!("Result for puzzle 1: {}", puzzle_one(&message, &sues));
    println!("Result for puzzle 2: {}", puzzle_two(&message, &sues));
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(message: &HashMap<&str, isize>, sues: &[HashMap<&str, isize>]) -> usize {
    'outer:
    for (idx, sue) in sues.iter().enumerate() {
        for (key, value) in sue {
            if let Some(message_value) = message.get(key) {
                if message_value != value {
                    continue 'outer;
                }
            } 
        }

        return idx + 1;
    }

    0
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two(message: &HashMap<&str, isize>, sues: &[HashMap<&str, isize>]) -> usize {
    'outer:
    for (idx, sue) in sues.iter().enumerate() {
        for (key, value) in sue {
            if let Some(message_value) = message.get(key) {
                match *key {
                    "cats" | "trees" => if message_value >= value { continue 'outer }
                    "pomeranians" | "goldfish" => if message_value <= value { continue 'outer }
                    _ => if message_value != value { continue 'outer; }
                }
            } 
        }

        return idx + 1;
    }

    0
}

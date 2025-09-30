// ---------------------------------------------------
//  Advent of Code 2024
//  Day 19: Linen Layout
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashMap;

const INPUT: &str = include_str!("day19.in");

pub fn run() {
    let mut lines = INPUT.lines();
    let towels: Vec<&str> = lines.next().unwrap().split(", ").collect();
    let patterns: Vec<&str> = lines.skip(1).collect();

    let (designs, ways) = puzzle_one_two(&towels, &patterns);

    println!("Result for puzzle 1: {}", designs);
    println!("Result for puzzle 2: {}", ways);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(towels: &[&str], patterns: &[&'static str]) -> (usize, usize) {
    let mut count_one = 0;
    let mut count_two = 0;

    for pattern in patterns {
        let p_towels: Vec<&str> = towels
            .iter()
            .filter_map(|&t| if pattern.contains(t) { Some(t) } else { None })
            .collect();
        let count = count_for(pattern, &p_towels, &mut HashMap::new());

        if count > 0 {
            count_one += 1;
            count_two += count;
        }
    }

    (count_one, count_two)
}

fn count_for(pattern: &'static str, towels: &[&str], seen: &mut HashMap<&'static str, usize>) -> usize {
    if let Some(count) = seen.get(&pattern) {
        return *count;
    }

    let count = towels
        .iter()
        .map(|&towel| {
            if pattern == towel {
                1
            } else if let Some(pattern) = pattern.strip_prefix(towel) {
                count_for(pattern, towels, seen)
            } else {
                0
            }
        })
        .sum();

    seen.insert(pattern, count);
    count
}

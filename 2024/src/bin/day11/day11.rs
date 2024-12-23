// ---------------------------------------------------
//  Advent of Code 2024
//  Day 11: Plutonian Pebbles
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashMap;

const INPUT: &str = include_str!("day11.in");

pub fn run() {
    let stones: Vec<usize> = INPUT
        .split_whitespace()
        .map(|stone| stone.parse().unwrap())
        .collect();

    println!("Result for puzzle 1: {}", puzzle_one_two(&stones, 25));
    println!("Result for puzzle 2: {}", puzzle_one_two(&stones, 75));
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(stones: &[usize], blinks: u32) -> usize {
    let mut stone_counts: HashMap<usize, usize> = stones.iter().map(|stone| (*stone, 1)).collect();

    for _ in 0..blinks {
        let mut new_counts = HashMap::new();

        for (stone, count) in stone_counts {
            let s = format!("{stone}");

            if stone == 0 {
                *new_counts.entry(1).or_default() += count;
            } else if s.len() % 2 == 0 {
                let (a, b) = s.split_at(s.len() / 2);

                *new_counts.entry(a.parse().unwrap()).or_default() += count;
                *new_counts.entry(b.parse().unwrap()).or_default() += count;
            } else {
                *new_counts.entry(stone * 2024).or_default() += count;
            }
        }

        stone_counts = new_counts;
    }

    stone_counts.values().sum()
}

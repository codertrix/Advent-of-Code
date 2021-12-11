// ---------------------------------------------------
//  Advent of Code 2021
//  Day 7: The Treachery of Whales
//
//  Copyright © 2021 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::cmp::min;

const INPUT: &str = include_str!("day07.in");

pub fn run() {
    let crabs: Vec<isize> = INPUT
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    let min_pos = *crabs.iter().min().unwrap();
    let max_pos = *crabs.iter().max().unwrap();
    let mut min_fuel1 = isize::MAX;
    let mut min_fuel2 = isize::MAX;

    for pos in min_pos..=max_pos {
        let mut fuel1 = 0;
        let mut fuel2 = 0;

        for crab in &crabs {
            let distance = (crab - pos).abs();
            fuel1 += distance;
            fuel2 += distance * (distance + 1) / 2;
        }

        min_fuel1 = min(min_fuel1, fuel1);
        min_fuel2 = min(min_fuel2, fuel2);
    }

    println!("Result for puzzle 1: {}", min_fuel1);
    println!("Result for puzzle 2: {}", min_fuel2);
}

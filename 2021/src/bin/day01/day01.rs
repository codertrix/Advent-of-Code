// ---------------------------------------------------
//  Advent of Code 2021
//  Day 1: Sonar Sweep
//
//  Copyright © 2021 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day01.in");

pub fn run() {
    let depths: Vec<usize> = INPUT
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("Result for puzzle 1: {}", increase_count(&depths, 1));
    println!("Result for puzzle 2: {}", increase_count(&depths, 3));
}


// --------------
//  Puzzle 1 + 2
// --------------

fn increase_count(depths: &[usize], offset: usize) -> usize {
    depths.windows(offset + 1)
        .map(|x| (x[0] < x[offset]) as usize)
        .sum()
}

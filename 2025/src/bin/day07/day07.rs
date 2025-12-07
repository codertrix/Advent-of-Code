// ---------------------------------------------------
//  Advent of Code 2025
//  Day 7: Laboratories
//
//  Copyright © 2025 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashMap;

const INPUT: &str = include_str!("day07.in");

pub fn run() {
    let lab: Vec<Vec<_>> = INPUT.lines().map(|line| line.chars().collect()).collect();
    let (result_one, result_two) = puzzle_one_two(&lab);

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(lab: &[Vec<char>]) -> (usize, usize) {
    let mut timelines = HashMap::new();
    let mut splits = 0;

    timelines.insert(lab[0].len() / 2, 1usize);

    for y in (2..lab.len()).step_by(2) {
        let mut next_timelines = HashMap::new();

        for (x, n) in &timelines {
            if lab[y][*x] == '^' {
                splits += 1;
                *next_timelines.entry(x - 1).or_default() += n;
                *next_timelines.entry(x + 1).or_default() += n;
            } else {
                *next_timelines.entry(*x).or_default() += n;
            }
        }

        timelines = next_timelines;
    }

    (splits, timelines.values().sum::<usize>())
}

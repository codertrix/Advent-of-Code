// ---------------------------------------------------
//  Advent of Code 2024
//  Day 1: Historian Hysteria
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashMap;

const INPUT: &str = include_str!("day01.in");

pub fn run() {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in INPUT.lines() {
        let mut items = line.split_ascii_whitespace();

        left.push(items.next().unwrap().parse().unwrap());
        right.push(items.next().unwrap().parse().unwrap());
    }

    println!("Result for puzzle 1: {}", puzzle_one(&mut left, &mut right));
    println!("Result for puzzle 2: {}", puzzle_two(&left, &right));
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(left: &mut [isize], right: &mut [isize]) -> isize {
    left.sort_unstable();
    right.sort_unstable();

    left.iter()
        .zip(right.iter())
        .fold(0, |total_distance, (l, r)| total_distance + (l - r).abs())
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two(left: &[isize], right: &[isize]) -> isize {
    let mut right_counts: HashMap<isize, isize> = HashMap::new();

    for r in right {
        right_counts
            .entry(*r)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    left.iter().fold(0, |similarity_score, l| {
        similarity_score + right_counts.get(l).unwrap_or(&0) * l
    })
}

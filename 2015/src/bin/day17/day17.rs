// ---------------------------------------------------
//  Advent of Code 2015
//  Day 17: No Such Thing as Too Much
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::cmp::Ordering;
use std::collections::HashMap;

const INPUT: &str = include_str!("day17.in");

pub fn run() {
    let results = puzzle_one_two();

    println!("Result for puzzle 1: {}", results.0);
    println!("Result for puzzle 2: {}", results.1);
}


// -------------
//  Puzzle 1 + 2
// -------------

fn store_in(to_store: usize, containers: &[usize], count: usize, container_counts: &mut HashMap<usize, usize>) -> usize {
    if containers.is_empty() {
        return 0;
    }

    let mut result = 0;

    for (idx, &container) in containers.iter().enumerate() {
        match container.cmp(&to_store) {
            Ordering::Equal => {
                *container_counts.entry(count).or_default() += 1;
                result += 1;
            }
            Ordering::Less => {
                result += store_in(to_store - container, &containers[(idx + 1)..], count + 1, container_counts);
            }
            _ => {}
        }
    }

    result
}

fn puzzle_one_two() -> (usize, usize) {
    let mut container_counts = HashMap::new();
    let containers: Vec<usize> = INPUT
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();

    let combination_count = store_in(150, &containers, 1, &mut container_counts);
    let &min_container_count = container_counts
        .get(container_counts.keys().min().unwrap())
        .unwrap();

    (combination_count, min_container_count)
}

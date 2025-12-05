// ---------------------------------------------------
//  Advent of Code 2025
//  Day 5: Cafeteria
//
//  Copyright © 2025 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::ops::RangeInclusive;

const INPUT: &str = include_str!("day05.in");

pub fn run() {
    let mut ranges = Vec::new();
    let mut ingredients = Vec::new();
    let mut input = INPUT.lines();

    for line in input.by_ref() {
        if line.is_empty() {
            break;
        }

        let (a, b) = line.split_once('-').unwrap();
        let range = a.parse().unwrap()..=b.parse().unwrap();

        ranges.push(range);
    }

    for line in input {
        ingredients.push(line.parse().unwrap());
    }

    println!("Result for puzzle 1: {}", puzzle_one(&ranges, &ingredients));
    println!("Result for puzzle 2: {}", puzzle_two(ranges));
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(ranges: &[RangeInclusive<u64>], ingredients: &[u64]) -> u64 {
    let mut fresh = 0;

    for ingredient in ingredients {
        for range in ranges {
            if range.contains(ingredient) {
                fresh += 1;
                break;
            }
        }
    }

    fresh
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two(mut ranges: Vec<RangeInclusive<u64>>) -> u64 {
    ranges.sort_unstable_by(|a, b| a.start().cmp(b.start()));

    let mut range_iter = ranges.into_iter();
    let mut r = range_iter.next().unwrap();
    let mut fresh = 0;

    for range in range_iter {
        if r.end() >= range.start() && r.end() < range.end() {
            r = *r.start()..=*range.end();
        } else if r.end() < range.start() {
            fresh += r.end() - r.start() + 1;
            r = range;
        }
    }

    fresh += r.end() - r.start() + 1;
    fresh
}

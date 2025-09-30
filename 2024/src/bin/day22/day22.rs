// ---------------------------------------------------
//  Advent of Code 2024
//  Day 22: Monkey Market
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("day22.in");

pub fn run() {
    let numbers: Vec<isize> = INPUT.lines().map(|n| n.parse().unwrap()).collect();
    let (result_one, result_two) = puzzle_one_two(numbers);

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(numbers: Vec<isize>) -> (isize, isize) {
    let mut result_one = 0;
    let mut result_two = HashMap::new();

    for mut number in numbers {
        let mut last_price = number % 10;
        let mut prices = Vec::new();
        let mut indexes = Vec::new();
        let mut changes = Vec::new();
        let mut seen = HashSet::new();

        for idx in 0..2000 {
            number = number ^ number << 6 & 0xffffff;
            number = number ^ number >> 5 & 0xffffff;
            number = number ^ number << 11 & 0xffffff;

            let price = number % 10;
            let change = price - last_price;

            if price == 9 && idx > 2 {
                indexes.push(idx as usize);
            }

            prices.push(price);
            changes.push(change);
            last_price = price;
        }

        result_one += number;

        for (idx, sequence) in changes.windows(4).enumerate() {
            if !seen.contains(sequence) {
                *result_two.entry(sequence.to_owned()).or_default() += prices[idx + 3];
                seen.insert(sequence);
            }
        }
    }

    (result_one, *result_two.values().max().unwrap())
}
